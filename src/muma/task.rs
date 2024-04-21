use crate::muma::Id;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task
{
    pub uuid: Id,
    pub task: String,
    pub parent: Option<Id>,
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

    #[allow(unused)]
    pub fn is_child(&self) -> bool
    {
        self.parent.is_some()
    }
}

