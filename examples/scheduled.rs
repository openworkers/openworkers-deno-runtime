use log::debug;
use log::error;
use openworkers_runtime::module_url;
use openworkers_runtime::AnyError;
use openworkers_runtime::ScheduledInit;
use openworkers_runtime::Task;
use openworkers_runtime::Worker;
use tokio::sync::oneshot;

fn get_path() -> String {
    std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("examples/scheduled.js"))
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    if !std::env::var("RUST_LOG").is_ok() {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();

    debug!("start main");

    // Check that the path is correct
    let file_path = {
        let path = get_path();
        if !std::path::Path::new(&path).is_file() {
            eprintln!("file not found: {}", path);
            std::process::exit(1);
        }
        path
    };

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<Option<AnyError>>();
    let (done_tx, done_rx) = oneshot::channel::<()>();

    let url = module_url(file_path.as_str());

    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    std::thread::spawn(move || Worker::new(url, shutdown_tx).exec(Task::Scheduled(Some(ScheduledInit::new(done_tx, time)))));

    debug!("js worker for {:?} started", file_path);

    // wait for completion signal
    match done_rx.await {
        Ok(()) => debug!("js task for {file_path} completed"),
        Err(err) => error!("js task for {file_path} did not complete: {err}"),
    }

    // wait for shutdown signal
    match shutdown_rx.await {
        Ok(None) => debug!("js worker for {file_path} stopped"),
        Ok(Some(err)) => error!("js worker for {file_path} error: {err}"),
        Err(err) => error!("js worker for {file_path} error: {err}"),
    }

    Ok(())
}
