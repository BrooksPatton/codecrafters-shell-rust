mod builtin_commands;
mod errors;
pub mod utilities;

#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::ffi::OsStrExt;

use anyhow::{Context, Result};

use crate::{
    builtin_commands::{BuiltinCommand, builtin_type::builtin_type, echo::echo},
    errors::CustomError,
    utilities::{get_command, get_path, print_error, print_prompt},
};

pub fn run() -> Result<()> {
    let path = get_path().context("Getting path")?;

    loop {
        print_prompt();

        let command = get_command().context("getting command")?;

        match command {
            BuiltinCommand::Echo(command_string) => echo(command_string.as_slice()),
            BuiltinCommand::Exit => break,
            BuiltinCommand::Type(arguments) => builtin_type(arguments, &path),
            BuiltinCommand::NotFound(command_string) => {
                let error = CustomError::CommandNotFound(command_string);
                print_error(error);
            }
        }
    }

    Ok(())
}
