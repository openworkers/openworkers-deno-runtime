use log::debug;
use log::error;
use openworkers_runtime::module_url;
use openworkers_runtime::AnyError;
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

    let url = module_url(file_path.as_str());

    std::thread::spawn(move || Worker::new(url, shutdown_tx).exec(Task::Scheduled));

    debug!("js worker for {:?} started", file_path);

    // wait for shutdown signal
    match shutdown_rx.await {
        Ok(None) => debug!("js worker for {:?} stopped", file_path),
        Ok(Some(err)) => error!("js worker for {:?} error: {}", file_path, err),
        Err(err) => error!("js worker for {:?} error: {}", file_path, err),
    }

    Ok(())
}
