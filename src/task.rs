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

pub enum Task {
    Fetch(FetchInit),
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
}
