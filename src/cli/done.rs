use clap::Args;
use crate::muma::error::*;
use colored::Colorize;
use crate::muma::TaskReg;

#[derive(Args)]
/// Mark a task as completed
pub struct DoneArgs
{
    #[arg(help=format!("Use {} to obtain a tasks id", format!("'{} {} {}'", "muma".blue(), "list".yellow(), "-a".yellow()).bold().italic()))]
    id: Vec<String>,

    // Marks a task as not completed
    #[arg(short='u', long="undone", default_value_t=false)]
    undone: bool,
}


impl DoneArgs
{
    pub fn run(&self) -> MumaResult<String>
    {
        for i in &self.id
        {
            match TaskReg::hid2id(i)
            {
                Some(id) => TaskReg::set_done(&id, !self.undone),
                None => return Ok(format!("{} Task with id '{}' not found", "Error:".bold().red(), i.red())),
            }?;
        }

        Ok(String::new())
    }
}
