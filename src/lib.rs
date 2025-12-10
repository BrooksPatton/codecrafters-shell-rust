mod builtin_commands;
mod command;
mod errors;
pub mod input_parser;
pub mod utilities;

use std::{io::Write, path::Path};

use crate::{
    builtin_commands::{
        BuiltinCommand, builtin_type::builtin_type, change_directory::change_directory, echo::echo,
        pwd::pwd, run_external_executable::run_external_executable,
    },
    errors::CustomError,
    utilities::{find_executable_file, get_command, get_path, print_prompt},
};
use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    let path = get_path().context("Getting path")?;
    let mut stdout: Vec<String> = vec![];
    let mut stderr: Vec<String> = vec![];

    loop {
        print_prompt();

        let command = get_command(&mut stderr).context("getting command")?;

        match command.builtin_command {
            BuiltinCommand::ChangeDirectory(arguments) => {
                change_directory(&arguments, &mut stderr)?
            }
            BuiltinCommand::Echo(command_string) => {
                echo(command_string.as_slice(), &mut stdout, &mut stderr)?;
            }
            BuiltinCommand::Exit => break,
            BuiltinCommand::PWD => pwd(&mut stdout, &mut stderr)?,
            BuiltinCommand::Type(arguments) => {
                builtin_type(arguments, &path, &mut stdout, &mut stderr)?;
            }
            BuiltinCommand::NotFound(command_string, arguments) => {
                if let Some(executable) = find_executable_file(&command_string, &path) {
                    run_external_executable(executable, &arguments, &mut stdout, &mut stderr)?;
                } else {
                    let error = CustomError::CommandNotFound(command_string);
                    stderr.push(format!("{error}"));
                }
            }
        }

        match command.standard_out {
            command::Output::Standard => {
                stdout
                    .iter()
                    .map(|message| message.trim())
                    .for_each(|message| print!("{message}"));

                if !stdout.is_empty() {
                    println!("");
                }
            }
            command::Output::File(input) => {
                let file_path = Path::new(&input);
                let mut file = std::fs::File::create(file_path)
                    .context("Creating standard out file as we're redirecting")?;

                stdout
                    .iter()
                    .try_for_each(|message| file.write_all(message.as_bytes()))?;
            }
        }

        match command.standard_error {
            command::Output::Standard => {
                stderr
                    .iter()
                    .map(|message| message.trim())
                    .for_each(|message| print!("{message}"));

                if !stderr.is_empty() {
                    println!("");
                }
            }
            command::Output::File(_) => todo!(),
        }

        stderr.clear();
        stdout.clear();
    }

    Ok(())
}
