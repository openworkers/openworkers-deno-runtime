use crate::ext::fetch_event_ext;
use crate::ext::permissions_ext;
use crate::ext::runtime_ext;
use crate::ext::scheduled_event_ext;

use crate::ext::Permissions;
use crate::Task;

use std::rc::Rc;

use deno_core::error::AnyError;
use deno_core::JsRuntime;

use deno_core::url::Url;
use deno_core::Snapshot;
use tokio::sync::oneshot;

use log::debug;

const USER_AGENT: &str = "OpenWorkers/0.1.0";

static RUNTIME_SNAPSHOT: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/RUNTIME_SNAPSHOT.bin"));

pub fn module_url(path_str: &str) -> Url {
    let current_dir = std::env::current_dir().unwrap();
    let current_dir = current_dir.as_path();
    deno_core::resolve_path(path_str, current_dir).unwrap()
}

pub(crate) fn runtime_snapshot() -> Option<Snapshot> {
    match RUNTIME_SNAPSHOT.len() {
        0 => None,
        _ => Some(Snapshot::Static(RUNTIME_SNAPSHOT)),
    }
}

pub(crate) fn extensions(for_snapshot: bool) -> Vec<deno_core::Extension> {
    let mut exts = vec![
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
        scheduled_event_ext::init_ops_and_esm(),
        runtime_ext::init_ops_and_esm(),
        permissions_ext::init_ops(),
    ];

    if !for_snapshot {
        return exts;
    }

    for ext in &mut exts {
        ext.js_files = std::borrow::Cow::Borrowed(&[]);
        ext.esm_files = std::borrow::Cow::Borrowed(&[]);
        ext.esm_entry_point = None;
    }

    exts
}

pub struct Worker {
    main_module: Url,
    js_runtime: deno_core::JsRuntime,
    shutdown_tx: oneshot::Sender<Option<AnyError>>,
}

impl Worker {
    pub fn new(main_module: Url, shutdown_tx: oneshot::Sender<Option<AnyError>>) -> Self {
        let mut js_runtime = match runtime_snapshot() {
            None => {
                debug!("no runtime snapshot");
                JsRuntime::new(deno_core::RuntimeOptions {
                    is_main: true,
                    extensions: extensions(false),
                    module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
                    startup_snapshot: None,
                    ..Default::default()
                })
            }
            Some(snapshot) => {
                debug!("using runtime snapshot");
                JsRuntime::new(deno_core::RuntimeOptions {
                    is_main: true,
                    extensions: extensions(true),
                    module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
                    startup_snapshot: Some(snapshot),
                    ..Default::default()
                })
            }
        };

        // Bootstrap
        {
            let script = format!("globalThis.bootstrap('{}')", USER_AGENT);

            match js_runtime.execute_script(
                deno_core::located_script_name!(),
                deno_core::ModuleCodeString::from(script),
            ) {
                Ok(_) => debug!("bootstrap succeeded"),
                Err(err) => panic!("bootstrap failed: {:?}", err)
            }
        }

        Self {
            js_runtime,
            main_module,
            shutdown_tx,
        }
    }

    pub fn exec(mut self, mut task: Task) {
        let future = async move {
            task.init(&mut self.js_runtime);

            let mod_id = self
                .js_runtime
                .load_main_module(&self.main_module, None)
                .await?;

            let result = self.js_runtime.mod_evaluate(mod_id);

            task.trigger(&mut self.js_runtime).expect("failed to trigger task")?;

            let opts = deno_core::PollEventLoopOptions {
                wait_for_inspector: false,
                pump_v8_message_loop: true,
            };

            self.js_runtime.run_event_loop(opts).await?;

            result.await
        };

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let local = tokio::task::LocalSet::new();
        match local.block_on(&runtime, future) {
            Ok(_) => {
                log::debug!("worker thread finished");
                self.shutdown_tx
                    .send(None)
                    .expect("failed to send shutdown signal");
            }
            Err(err) => {
                log::error!("worker thread failed {:?}", err);
                self.shutdown_tx
                    .send(Some(err))
                    .expect("failed to send shutdown signal");
            }
        }
    }
}
