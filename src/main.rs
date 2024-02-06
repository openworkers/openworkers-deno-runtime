// main.rs
mod runtime;
mod permissions;

use permissions::Permissions;

use deno_core::{error::AnyError, FsModuleLoader};
use std::rc::Rc;

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let current_dir = std::env::current_dir()?;
    let main_module = deno_core::resolve_path(file_path, current_dir.as_path()).unwrap();

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
        runtime::runtime::init_ops_and_esm(),
    ];

    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        is_main: true,
        extensions,
        module_loader: Some(Rc::new(FsModuleLoader)),
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

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);

    js_runtime
        .run_event_loop(deno_core::PollEventLoopOptions {
            wait_for_inspector: false,
            pump_v8_message_loop: false,
        })
        .await?;
    result.await
}

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    if let Err(error) = runtime.block_on(run_js("./example.js")) {
        eprintln!("error: {}", error);
    }
}
