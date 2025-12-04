mod builtin_commands;
mod errors;
pub mod utilities;

use crate::{
    builtin_commands::{
        BuiltinCommand, builtin_type::builtin_type, echo::echo,
        run_external_executable::run_external_executable,
    },
    errors::CustomError,
    utilities::{find_executable_file, get_command, get_path, print_error, print_prompt},
};
use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    let path = get_path().context("Getting path")?;

    echo(&["paths:", &format!("{path:?}")]);

    loop {
        print_prompt();

        let command = get_command().context("getting command")?;

        match command {
            BuiltinCommand::Echo(command_string) => echo(command_string.as_slice()),
            BuiltinCommand::Exit => break,
            BuiltinCommand::Type(arguments) => builtin_type(arguments, &path),
            BuiltinCommand::NotFound(command_string, arguments) => {
                if let Some(executable) = find_executable_file(&command_string, &path) {
                    run_external_executable(executable, &arguments);
                } else {
                    let error = CustomError::CommandNotFound(command_string);
                    print_error(error);
                }
            }
        }
    }

    Ok(())
}
