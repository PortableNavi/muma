mod add;
mod done;
mod remove;
mod list;


use clap::{Parser, Subcommand};
use crate::muma::error::MumaResult;
pub use list::list;


#[derive(Parser)]
/// Muma lets you manage a simple todo list trough a cli
pub struct Cli
{
    #[command(subcommand)]
    pub subcommand: Option<Subcommands>,
}


#[derive(Subcommand)]
pub enum Subcommands
{
    Add(add::AddArgs),
    Done(done::DoneArgs),
    Remove(remove::RemoveArgs),
    List(list::ListArgs),
}


impl Subcommands
{
    pub fn run(&self) -> MumaResult<String>
    {
        match self
        {
            Self::Add(arg) => arg.run(),
            Self::Done(arg) => arg.run(),
            Self::Remove(arg) => arg.run(),
            Self::List(arg) => arg.run(),
        }
    }
}

