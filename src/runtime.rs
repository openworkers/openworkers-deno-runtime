use crate::ext::fetch_event_ext;
use crate::ext::permissions_ext;
use crate::ext::runtime_ext;

use crate::ext::Permissions;
use crate::task::TaskType;
use crate::Task;

use std::rc::Rc;

use deno_core::error::AnyError;

use deno_core::Snapshot;
use tokio::sync::oneshot;

use log::{debug, error};

const USER_AGENT: &str = "OpenWorkers/0.1.0";

static RUNTIME_SNAPSHOT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/RUNTIME_SNAPSHOT.bin"));

pub (crate) fn extensions() -> Vec<deno_core::Extension> {
    vec![
        deno_webidl::deno_webidl::init_ops_and_esm(),
        deno_console::deno_console::init_ops_and_esm(),
        deno_url::deno_url::init_ops_and_esm(),
        deno_web::deno_web::init_ops_and_esm::<Permissions>(
            std::sync::Arc::new(deno_web::BlobStore::default()),
            None,
        ),
        deno_crypto::deno_crypto::init_ops_and_esm(None),
        deno_fetch::deno_fetch::init_ops_and_esm::<Permissions>(deno_fetch::Options {
            user_agent: USER_AGENT.to_string(),
            ..Default::default()
        }),

        // OpenWorkers extensions
        fetch_event_ext::init_ops_and_esm(),
        runtime_ext::init_ops_and_esm(),
        permissions_ext::init_ops(),
    ]
}

pub fn run_js(path_str: &str, task: Task, shutdown_tx: oneshot::Sender<Option<AnyError>>) {
    let current_dir = std::env::current_dir().unwrap();
    let current_dir = current_dir.as_path();
    let main_module = deno_core::resolve_path(path_str, current_dir).unwrap();

    let snapshot = match RUNTIME_SNAPSHOT.len() {
        0 => None,
        _ => Some(Snapshot::Static(RUNTIME_SNAPSHOT)),
    };

    let mut js_runtime = match snapshot {
        None => {
            deno_core::JsRuntime::new(deno_core::RuntimeOptions {
                is_main: true,
                extensions: extensions(),
                module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
                startup_snapshot: None,
                ..Default::default()
            })
        }
        Some(snapshot) => {
            deno_core::JsRuntime::new(
                deno_core::RuntimeOptions {
                    is_main: true,
                    extensions: {
                        let mut exts = extensions();
                        
                        for ext in &mut exts {
                            ext.js_files = std::borrow::Cow::Borrowed(&[]);
                            ext.esm_files = std::borrow::Cow::Borrowed(&[]);
                            ext.esm_entry_point = None;
                        }

                        exts
                    },
                    // module_loader: None,
                    module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
                    startup_snapshot: Some(snapshot),
                    ..Default::default()
                }
            )
        }
    };

    // Bootstrap
    {
        let script = format!("globalThis.bootstrap('{}')", USER_AGENT);

        js_runtime
            .execute_script(
                deno_core::located_script_name!(),
                deno_core::ModuleCodeString::from(script),
            )
            .unwrap();
    }

    let task_type = task.task_type();

    {
        match task {
            Task::Fetch(evt) => {
                debug!("set fetch request");

                let op_state_rc = js_runtime.op_state();
                let mut op_state = op_state_rc.borrow_mut();

                op_state.put(evt);
            }
            Task::Scheduled => {}
            Task::None => {}
        }
    }

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let future = async move {
        let mod_id = js_runtime.load_main_module(&main_module, None).await?;
        let result = js_runtime.mod_evaluate(mod_id);

        if !task_type.is_none() {
            js_runtime
                .execute_script(
                    deno_core::located_script_name!(),
                    deno_core::ModuleCodeString::from(match task_type {
                        TaskType::Fetch => format!("globalThis.triggerFetchEvent()"),
                        TaskType::Scheduled => format!("globalThis.triggerScheduledEvent(Date.now())"),
                        TaskType::None => unreachable!(),
                    }),
                )
                .unwrap();
        }

        let opts = deno_core::PollEventLoopOptions {
            wait_for_inspector: false,
            pump_v8_message_loop: true,
        };

        js_runtime.run_event_loop(opts).await?;

        result.await
    };

    let local = tokio::task::LocalSet::new();
    match local.block_on(&runtime, future) {
        Ok(_) => {
            debug!("worker thread finished");
            shutdown_tx
                .send(None)
                .expect("failed to send shutdown signal");
        }
        Err(err) => {
            error!("worker thread failed {:?}", err);
            shutdown_tx
                .send(Some(err))
                .expect("failed to send shutdown signal");
        }
    }
}
