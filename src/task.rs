use deno_core::error::AnyError;
use deno_core::v8::Global;
use deno_core::v8::Value;
use deno_core::JsRuntime;

use crate::FetchInit;
use crate::ScheduledInit;

pub enum TaskType {
    Fetch,
    Scheduled,
    Noop,
}

pub trait TaskTrigger {
    fn trigger(&self, _js_runtime: &mut JsRuntime) -> Option<Result<Global<Value>, AnyError>> {
        None
    }
}

pub trait TaskInit {
    fn init(&self, _js_runtime: &mut JsRuntime);
}

pub enum Task {
    Fetch(Option<FetchInit>),
    Scheduled(Option<ScheduledInit>),
    Noop(Option<tokio::sync::oneshot::Sender<()>>),
}

impl Task {
    pub fn task_type(&self) -> TaskType {
        match self {
            Task::Fetch(_) => TaskType::Fetch,
            Task::Scheduled(_) => TaskType::Scheduled,
            Task::Noop(_) => TaskType::Noop,
        }
    }

    pub fn init(&mut self, js_runtime: &mut JsRuntime) {
        match self {
            Task::Fetch(data) => {
                let op_state_rc = js_runtime.op_state();
                let mut op_state = op_state_rc.borrow_mut();
                op_state.put(data.take().unwrap());
            }
            Task::Scheduled(data) => {
                let op_state_rc = js_runtime.op_state();
                let mut op_state = op_state_rc.borrow_mut();
                op_state.put(data.take().unwrap());
            }
            Task::Noop(_) => {}
        }
    }

    pub fn trigger(&self, js_runtime: &mut JsRuntime) -> Option<Result<Global<Value>, AnyError>> {
        match self {
            Task::Fetch(_) => Some(js_runtime.execute_script(
                deno_core::located_script_name!(),
                deno_core::ModuleCodeString::from(format!("globalThis.triggerFetchEvent()")),
            )),
            Task::Scheduled(_) => Some(js_runtime.execute_script(
                deno_core::located_script_name!(),
                deno_core::ModuleCodeString::from(format!("globalThis.triggerScheduledEvent()")),
            )),
            Task::Noop(_) => None,
        }
    }
}
