use log::debug;
use log::error;
use openworkers_runtime::run_js;
use openworkers_runtime::AnyError;
use openworkers_runtime::Task;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() -> Result<(), ()> {
    if !std::env::var("RUST_LOG").is_ok() {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();

    debug!("start main");

    let file_path = String::from("examples/scheduled.js");

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<Option<AnyError>>();

    let _res = {
        let file_path = file_path.clone();

        std::thread::spawn(move || run_js(file_path.as_str(), Task::Scheduled, shutdown_tx))
    };

    debug!("js worker for {:?} started", file_path);

    // wait for shutdown signal
    match shutdown_rx.await {
        Ok(None) => debug!("js worker for {:?} stopped", file_path),
        Ok(Some(err)) => {
            error!("js worker for {:?} error: {}", file_path, err);
        }
        Err(err) => {
            error!("js worker for {:?} error: {}", file_path, err);
        }
    }

    Ok(())
}
