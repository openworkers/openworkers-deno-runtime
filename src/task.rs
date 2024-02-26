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
}
