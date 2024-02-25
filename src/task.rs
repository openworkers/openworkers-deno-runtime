use deno_core::error::AnyError;
use deno_core::v8::Global;
use deno_core::v8::Value;
use deno_core::JsRuntime;

use crate::FetchInit;

pub enum TaskType {
    Fetch,
    Scheduled,
    None,
}

impl TaskType {
    pub fn is_none(&self) -> bool {
        match self {
            TaskType::None => true,
            _ => false,
        }
    }
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
    Scheduled,
    None,
}

impl Task {
    pub fn task_type(&self) -> TaskType {
        match self {
            Task::Fetch(_) => TaskType::Fetch,
            Task::Scheduled => TaskType::Scheduled,
            Task::None => TaskType::None,
        }
    }

    pub fn init(&mut self, js_runtime: &mut JsRuntime) {
        match self {
            Task::Fetch(data) => {
                let op_state_rc = js_runtime.op_state();
                let mut op_state = op_state_rc.borrow_mut();

                let fetch_init = data.take().unwrap();

                op_state.put(fetch_init);
            }
            Task::Scheduled => {}
            Task::None => {}
        }
    }

    pub fn trigger(&self, js_runtime: &mut JsRuntime) -> Option<Result<Global<Value>, AnyError>> {
        match self {
            Task::Fetch(_) => Some(js_runtime.execute_script(
                deno_core::located_script_name!(),
                deno_core::ModuleCodeString::from(format!("globalThis.triggerFetchEvent()")),
            )),
            Task::Scheduled => Some(js_runtime.execute_script(
                deno_core::located_script_name!(),
                deno_core::ModuleCodeString::from(format!("globalThis.triggerScheduledEvent()")),
            )),
            Task::None => None,
        }
    }
}
