use std::rc::Rc;

use deno_core::error::AnyError;
use deno_core::op2;
use deno_core::serde::Serialize;
use deno_core::Extension;
use deno_core::ExtensionFileSource;
use deno_core::OpState;
use deno_core::ResourceId;
use log::debug;

type ResponseSender = tokio::sync::oneshot::Sender<()>;

#[derive(Debug)]
pub struct ScheduledInit {
    pub(crate) res_tx: Option<ResponseSender>,
    pub(crate) time: u64,
}

impl ScheduledInit {
    pub fn new(res_tx: ResponseSender, time: u64) -> Self {
        ScheduledInit {
            res_tx: Some(res_tx),
            time,
        }
    }
}

#[derive(Debug)]
struct ScheduledTx {
    tx: ResponseSender,
}

impl From<ResponseSender> for ScheduledTx {
    fn from(tx: ResponseSender) -> Self {
        ScheduledTx { tx }
    }
}

impl ScheduledTx {
    pub fn send(self) -> Result<(), ()> {
        self.tx.send(())
    }
}

#[derive(Debug, Serialize)]
struct ScheduledEvent {
    rid: u32,
    time: u64,
}

deno_core::extension!(
    scheduled_event,
    deps = [deno_console, deno_fetch],
    ops = [op_scheduled_init, op_scheduled_respond],
    customizer = |ext: &mut Extension| {
        ext.esm_files.to_mut().push(ExtensionFileSource::new(
            "ext:event_scheduled.js",
            include_str!("event_scheduled.js"),
        ));
        ext.esm_entry_point = Some("ext:event_scheduled.js");
    }
);

impl deno_core::Resource for ScheduledTx {
    fn close(self: Rc<Self>) {
        println!("TODO Resource.close impl for ScheduledTx"); // TODO
    }
}

#[op2]
#[serde]
fn op_scheduled_init(state: &mut OpState) -> Result<ScheduledEvent, AnyError> {
    debug!("op_scheduled_init");

    let mut evt = state.take::<ScheduledInit>();

    let time = evt.time;

    let res = ScheduledTx::from(evt.res_tx.take().unwrap());

    let rid = state.resource_table.add::<ScheduledTx>(res);

    Ok(ScheduledEvent { rid, time })
}

#[op2]
#[serde]
fn op_scheduled_respond(state: &mut OpState, #[smi] rid: ResourceId) -> Result<(), AnyError> {
    debug!("op_scheduled_respond");

    match state.resource_table.take::<ScheduledTx>(rid) {
        Ok(tx) => Ok(Rc::try_unwrap(tx).unwrap().send().unwrap()),
        Err(err) => Err(err),
    }
}
