mod commands;
mod errors;
pub mod utilities;

#[allow(unused_imports)]
use std::io::{self, Write};

use anyhow::{Context, Result};

use crate::{
    commands::{Command, builtin_type::builtin_type, echo::echo},
    errors::CustomError,
    utilities::{get_command, print_error, print_prompt},
};

pub fn run() -> Result<()> {
    loop {
        print_prompt();

        let command = get_command().context("getting command")?;

        match command {
            Command::Echo(command_string) => echo(&command_string),
            Command::Exit => break,
            Command::Type(arguments) => builtin_type(arguments),
            Command::NotFound(command_string) => {
                let error = CustomError::CommandNotFound(command_string);
                print_error(error);
            }
        }
    }

    Ok(())
}
