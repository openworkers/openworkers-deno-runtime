use deno_core::Extension;
use deno_core::ExtensionFileSource;
use deno_core::OpState;
use deno_core::serde::Serialize;

deno_core::extension!(
    runtime,
    deps = [
        deno_console,
        deno_web,
        deno_crypto,
        deno_fetch,
        fetch_event,
        scheduled_event
    ],
    ops = [op_log],
    customizer = |ext: &mut Extension| {
        ext.esm_files.to_mut().push(ExtensionFileSource::new(
            "ext:runtime.js",
            include_str!("runtime.js"),
        ));
        ext.esm_entry_point = Some("ext:runtime.js");
    }
);

#[derive(Debug, Serialize)]
pub struct LogEvent {
    pub level: String,
    pub message: String,
}

#[deno_core::op2(fast)]
fn op_log(state: &mut OpState, #[string] level: &str, #[string] message: &str) {
    let evt = LogEvent {
        level: level.to_string(),
        message: message.to_string(),
    };

    log::debug!("op_log {:?}", evt);

    let tx = state.try_borrow_mut::<std::sync::mpsc::Sender<LogEvent>>();

    match tx {
        None => log::warn!("failed to borrow log event sender"),
        Some(tx) => match tx.send(evt) {
            Ok(_) => {},
            Err(_) => log::error!("failed to send log event"),
        },
    }
}
