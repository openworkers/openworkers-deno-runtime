// main.rs

mod ext;
mod runtime;
mod permissions;

use log::debug;

#[tokio::main]
async fn main() -> Result<(), ()> {
    debug!("start main");

    env_logger::init();

    let path = String::from("example.js");

    runtime::serve(path).await;

    Ok(())
}
