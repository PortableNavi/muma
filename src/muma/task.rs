use crate::muma::{error::MumaResult, error::MumaError, Id, TaskReg};
use serde::{Serialize, Deserialize};
use uuid::Uuid;


/// Desribes a Task entry in the todo list.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task
{
    /// Unique ID
    pub uuid: Id,

    /// Text describing the task
    pub task: String,

    pub parent: Option<Id>,

    /// A list of children tasks
    pub children: Vec<Id>,

    pub done: bool,
}


impl Task
{
    pub fn new_toplevel(task: impl Into<String>) -> Self
    {
        Self {
            uuid: Id::new(),
            task: task.into(),
            parent: None,
            children: vec![],
            done: false,
        }
    }

    pub fn new_child(task: impl Into<String>, parent: Id) -> Self
    {
        Self {
            uuid: Id::new(),
            task: task.into(),
            parent: Some(parent),
            children: vec![],
            done: false,
        }
    }

    pub fn is_child(&self) -> bool
    {
        self.parent.is_some()
    }
}

