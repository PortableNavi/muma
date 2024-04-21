#![allow(clippy::suspicious_else_formatting)]

mod cli;
mod muma;

use cli::Cli;
use clap::Parser;
use colored::Colorize;
use muma::{TaskReg, error::{MumaResult, MumaError}};


fn main()
{
    if let Err(e) = run()
    {
        println!("{}", match e
        {
            MumaError::IO(e) => report_error(format!("Failed to read/write file: {e}")),
            MumaError::MalformedId(_) => report_error("Encountered a malformed task id"),
            MumaError::TomlDeError(e) => report_error(format!("Toml parsing error: {e}")),
            MumaError::TomlSerError(e) => report_error(format!("Toml parsing error: {e}")), 
            MumaError::TaskDoesNotExist(_) => report_error(e),
        });
    }
}


fn run() -> MumaResult<()>
{
    TaskReg::init()?;
    let cli = cli::Cli::parse();

    if let Some(cmd) = &cli.subcommand
    {
        let result = cmd.run()?;

        if !result.is_empty()
        {
            println!("{result}");
        }
    }

    else
    {
        println!("{}", cli::list(true, false)?);
    }

    TaskReg::get().save(TaskReg::default_path()?)
}


pub fn report_error(msg: impl std::fmt::Display) -> String
{
    format!("{} {}", "Error:".bold().red(), msg)
}

