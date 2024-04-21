use clap::Args;
use crate::muma::error::*;
use crate::muma::{Task, TaskReg};
use colored::Colorize;


#[derive(Args)]
/// Add a new task to the todo list
pub struct AddArgs
{
    /// Description/Name of the Task 
    task: Vec<String>,

    /// Add this task as a child of another task
    #[arg(short='p', long="parent")]
    parent: Option<String>,
}


impl AddArgs
{
    pub fn run(&self) -> MumaResult<String>
    {
        let tasks = match &self.parent
        {
            Some(parent) => {
                match TaskReg::hid2id(parent)
                {
                    Some(id) => self.task.iter().map(|t| Task::new_child(t.clone(), id)).collect::<Vec<_>>(),
                    None => return Ok(format!("{} Task with id '{}' not found", "Error:".bold().red(), parent.red())),
                }
            },

            None => self.task.iter().map(|t| Task::new_toplevel(t.clone())).collect::<Vec<_>>(),
        };
        
        for t in tasks
        {
            TaskReg::add_task(t)?;
        }

        Ok(String::new())
    }
}
