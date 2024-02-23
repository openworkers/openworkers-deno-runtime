// main.rs

mod ext;
mod runtime;

use log::debug;

#[tokio::main]
async fn main() -> Result<(), ()> {
    if !std::env::var("RUST_LOG").is_ok() {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();

    debug!("start main");

    let path = String::from("example.js");

    runtime::serve(path).await;

    Ok(())
}
