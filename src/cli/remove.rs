use clap::Args;
use crate::muma::error::*;
use colored::Colorize;
use crate::muma::{Task, TaskReg};


#[derive(Args)]
/// Remove a task from the todo list
pub struct RemoveArgs
{
    #[arg(help=format!("Use \"{}\" to obtain a tasks id", "muma list -a".bold().blue()))]
    id: Vec<String>,
}


impl RemoveArgs
{
    pub fn run(&self) -> MumaResult<String>
    {
        for i in &self.id
        {
            let id = match TaskReg::hid2id(i)
            {
                Some(id) => {
                    match TaskReg::remove_task(&id)
                    {
                        Err(MumaError::TaskDoesNotExist(_)) => Ok(()),
                        e => e
                    }?
                },
                None => return Ok(format!("{} Task with id '{}' not found", "Error:".bold().red(), i.red())),
            };
        }
        
        Ok(String::new())
    }
}
