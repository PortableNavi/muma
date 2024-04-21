use clap::Args;
use crate::muma::error::*;
use crate::muma::TaskReg;
use colored::Colorize;


#[derive(Args)]
/// Display your todo list
pub struct ListArgs 
{
    /// Also display tasks marked as done
    #[arg(short='a', default_value_t=false)]
    all: bool,

    /// Only display tasks that are marked as done
    #[arg(short='c', default_value_t=false)]
    completed: bool,
}


impl ListArgs
{
    pub fn run(&self) -> MumaResult<String>
    {
        list(self.all, self.completed)
    }
}


pub fn list(all: bool, completed: bool) -> MumaResult<String>
{
    let mut output = String::new();
    let hid = TaskReg::copy_hid();
    
    for (h, i) in hid.iter()
    {
        let task = TaskReg::get().get_task_mut(i).unwrap().clone();

        let prefix_len = h.chars().filter(|e| e==&'.').count() * 2;
        let prefix = String::from_utf8(vec![b' '; prefix_len]).unwrap();

        if task.done && (all || completed)
        {
            output.push_str(&format!("{prefix}{} {} {}\n", "✔".bold().green(), h.bold(), task.task.italic().bright_black().strikethrough()));
        }

        if !task.done && !completed
        {
            output.push_str(&format!("{prefix}{} {} {}\n", "•".bright_black(), h.bold(), task.task));
        }
    }

    Ok(output)

}
