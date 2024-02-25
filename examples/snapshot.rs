use log::debug;
use log::error;
use openworkers_runtime::run_js;
use openworkers_runtime::AnyError;
use openworkers_runtime::Task;
use tokio::sync::oneshot;

fn main() {
    openworkers_runtime::create_runtime_snapshot();
}
