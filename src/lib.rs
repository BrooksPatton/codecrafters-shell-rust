mod builtin_commands;
mod command;
mod errors;
pub mod input_parser;
pub mod utilities;

use std::{
    io::pipe,
    os::fd::IntoRawFd,
    sync::mpsc::{channel, sync_channel},
};

use crate::{
    builtin_commands::{
        BuiltinCommand, builtin_type::builtin_type, change_directory::change_directory, echo::echo,
        pwd::pwd, run_external_executable::run_external_executable,
    },
    errors::CustomError,
    utilities::{find_executable_file, get_command, get_path, print_error, print_prompt},
};
use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    let path = get_path().context("Getting path")?;
    let (mut writer, reader) = channel::<String>();

    loop {
        print_prompt();

        let command = get_command().context("getting command")?;

        match command.builtin_command {
            BuiltinCommand::ChangeDirectory(arguments) => change_directory(&arguments)?,
            BuiltinCommand::Echo(command_string) => echo(command_string.as_slice(), &mut writer),
            BuiltinCommand::Exit => break,
            BuiltinCommand::PWD => pwd(&mut writer)?,
            BuiltinCommand::Type(arguments) => builtin_type(arguments, &path, &mut writer),
            BuiltinCommand::NotFound(command_string, arguments) => {
                if let Some(executable) = find_executable_file(&command_string, &path) {
                    run_external_executable(executable, &arguments);
                } else {
                    let error = CustomError::CommandNotFound(command_string);
                    print_error(error);
                }
            }
        }

        // print what we have in our pipe
        match command.standard_out {
            command::Output::StandardOut => {
                for message in reader.try_iter() {
                    println!("{message}");
                }
            }
            command::Output::File(_) => todo!(),
        }
    }

    Ok(())
}
