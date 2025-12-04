mod builtin_commands;
mod errors;
pub mod utilities;

use crate::{
    builtin_commands::{BuiltinCommand, builtin_type::builtin_type, echo::echo},
    errors::CustomError,
    utilities::{find_executable_file, get_command, get_path, print_error, print_prompt},
};
use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    let path = get_path().context("Getting path")?;

    loop {
        print_prompt();

        let command = get_command().context("getting command")?;

        match command {
            BuiltinCommand::Echo(command_string) => echo(command_string.as_slice()),
            BuiltinCommand::Exit => break,
            BuiltinCommand::Type(arguments) => builtin_type(arguments, &path),
            BuiltinCommand::NotFound(command_string, arguments) => {
                if let Some(executable) = find_executable_file(&command_string, &path) {
                    println!("");
                    let path = executable.path();
                    let mut command = std::process::Command::new(path);

                    command.args(&arguments);

                    let Ok(mut process_child) = command.spawn() else {
                        continue;
                    };
                    let Ok(_result) = process_child.wait() else {
                        continue;
                    };
                } else {
                    let error = CustomError::CommandNotFound(command_string);
                    print_error(error);
                }
            }
        }
    }

    Ok(())
}
