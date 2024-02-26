use deno_core::error::AnyError;
use deno_core::v8::Global;
use deno_core::v8::Value;
use deno_core::JsRuntime;
use deno_core::ModuleCodeString;
use deno_core::ResourceId;

use crate::FetchInit;
use crate::ScheduledInit;

#[derive(Debug)]
pub enum TaskType {
    Fetch,
    Scheduled,
}

pub enum Task {
    Fetch(Option<FetchInit>),
    Scheduled(Option<ScheduledInit>),
}

impl Task {
    pub fn task_type(&self) -> TaskType {
        match self {
            Task::Fetch(_) => TaskType::Fetch,
            Task::Scheduled(_) => TaskType::Scheduled,
        }
    }

    fn init(&mut self, js_runtime: &mut JsRuntime) -> ResourceId {
        let op_state_rc = js_runtime.op_state();
        let mut op_state = op_state_rc.borrow_mut();

        match self {
            Task::Fetch(data) => op_state.resource_table.add(data.take().unwrap()),
            Task::Scheduled(data) => op_state.resource_table.add(data.take().unwrap()),
        }
    }

    pub fn trigger(
        &mut self,
        js_runtime: &mut JsRuntime,
    ) -> Option<Result<Global<Value>, AnyError>> {
        let rid = self.init(js_runtime);

        match self {
            Task::Fetch(_) => Some(js_runtime.execute_script(
                deno_core::located_script_name!(),
                ModuleCodeString::from(format!("globalThis.triggerFetchEvent({rid})")),
            )),
            Task::Scheduled(_) => Some(js_runtime.execute_script(
                deno_core::located_script_name!(),
                ModuleCodeString::from(format!("globalThis.triggerScheduledEvent({rid})")),
            )),
        }
    }
}
