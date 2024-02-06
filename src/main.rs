// main.rs
use deno_core::{error::AnyError, FsModuleLoader};
use openworkers_deno_runtime::runtime::runtime;
use std::rc::Rc;

#[derive(Clone)]
pub struct Permissions {}

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let current_dir = std::env::current_dir()?;
    let main_module = deno_core::resolve_path(file_path, current_dir.as_path()).unwrap();

    let extensions = vec![
        deno_console::deno_console::init_ops_and_esm(),
        runtime::init_ops_and_esm(),
    ];

    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        is_main: true,
        extensions,
        module_loader: Some(Rc::new(FsModuleLoader)),
        ..Default::default()
    });

    // Bootstrap the runtime
    {
        // Bootstrapping stage
        let script = format!("globalThis.bootstrap()");

        js_runtime
            .execute_script(deno_core::located_script_name!(), deno_core::ModuleCodeString::from(script))
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
