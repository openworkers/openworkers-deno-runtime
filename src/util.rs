use deno_core::v8;

use crate::Task;
use crate::Worker;

pub(crate) fn extract_trigger<'a>(
    name: &str,
    scope: &mut v8::HandleScope<'a>,
    object: v8::Local<'a, v8::Object>,
) -> Option<v8::Global<v8::Function>> {
    let key = v8::String::new(scope, name).unwrap().into();

    let ret = match object.get(scope, key) {
        Some(fetch) => fetch,
        None => return None,
    };

    let ret: v8::Local<v8::Function> = match ret.try_into() {
        Ok(ret) => ret,
        Err(_) => return None,
    };

    Some(v8::Global::new(scope, ret))
}

pub(crate) fn exec_task(worker: &mut Worker, task: &mut Task) {
    let rid = {
        let op_state_rc = worker.js_runtime.op_state();
        let mut op_state = op_state_rc.borrow_mut();

        match task {
            Task::Fetch(data) => op_state.resource_table.add(data.take().unwrap()),
            Task::Scheduled(data) => op_state.resource_table.add(data.take().unwrap()),
        }
    };

    let scope = &mut worker.js_runtime.handle_scope();

    let trigger = v8::Local::new(
        scope,
        match task {
            Task::Fetch(_) => &worker.trigger_fetch,
            Task::Scheduled(_) => &worker.trigger_scheduled,
        },
    );

    let recv = v8::undefined(scope);

    let rid = v8::Integer::new(scope, rid as i32).into();

    match trigger.call(scope, recv.into(), &[rid]) {
        Some(_) => log::debug!("successfully called trigger"),
        None => log::error!("failed to call trigger"),
    };
}
