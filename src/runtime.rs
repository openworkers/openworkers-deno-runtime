use crate::ext::runtime_ext;
use crate::ext::fetch_init_ext;

use crate::permissions::Permissions;

use std::rc::Rc;

use tokio::sync::oneshot;

use log::{debug, error};

pub fn run_js(path_str: &str, shutdown_tx: oneshot::Sender<()>) {
    let current_dir = std::env::current_dir().unwrap();
    let current_dir = current_dir.as_path();
    let main_module = deno_core::resolve_path(path_str, current_dir).unwrap();

    let user_agent = "OpenWorkers/0.1.0";

    let extensions = vec![
        deno_webidl::deno_webidl::init_ops_and_esm(),
        deno_console::deno_console::init_ops_and_esm(),
        deno_url::deno_url::init_ops_and_esm(),
        deno_web::deno_web::init_ops_and_esm::<Permissions>(
            std::sync::Arc::new(deno_web::BlobStore::default()),
            None,
        ),
        deno_crypto::deno_crypto::init_ops_and_esm(None),
        deno_fetch::deno_fetch::init_ops_and_esm::<Permissions>(deno_fetch::Options {
            user_agent: user_agent.to_string(),
            ..Default::default()
        }),
        fetch_init_ext::init_ops_and_esm(),
        runtime_ext::init_ops_and_esm(),
    ];

    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        is_main: true,
        extensions,
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        ..Default::default()
    });

    // Bootstrap
    {
        let script = format!("globalThis.bootstrap('{}')", user_agent);

        js_runtime
            .execute_script(
                deno_core::located_script_name!(),
                deno_core::ModuleCodeString::from(script),
            )
            .unwrap();
    }

    // Set fetch request
    {
        debug!("set fetch request");

        let op_state_rc = js_runtime.op_state();
        let mut op_state = op_state_rc.borrow_mut();

        let fetch_resource = crate::ext::FetchResource {
            req: String::from("fetch-request")
        };

        op_state.put(fetch_resource);
    };

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let future = async move {
        let mod_id = js_runtime.load_main_module(&main_module, None).await?;
        let result = js_runtime.mod_evaluate(mod_id);

        let opts = deno_core::PollEventLoopOptions {
            wait_for_inspector: false,
            pump_v8_message_loop: false,
        };

        js_runtime.run_event_loop(opts).await?;

        result.await
    };

    let local = tokio::task::LocalSet::new();
    match local.block_on(&runtime, future) {
        Ok(_) => debug!("worker thread finished"),
        Err(err) => error!("worker thread failed {:?}", err),
    }

    shutdown_tx
        .send(())
        .expect("failed to send shutdown signal");
}

pub async fn serve(file_path: String) {
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    let res = {
        let file_path = file_path.clone();

        std::thread::spawn(move || run_js(file_path.as_str(), shutdown_tx))
    };

    debug!("js worker for {:?} started {:?}", file_path, res);

    // wait for shutdown signal
    let res = shutdown_rx.await;

    debug!("js worker for {:?} stopped {:?}", file_path, res);
}
