mod builtin_commands;
mod command;
mod errors;
pub mod input_parser;
pub mod utilities;

use std::{io::Write, path::Path, sync::mpsc::channel};

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
    let (mut stdout_writer, stdout_reader) = channel::<String>();
    let (mut stderror_writer, stderror_reader) = channel::<String>();

    loop {
        print_prompt();

        let command = get_command(&mut stderror_writer).context("getting command")?;

        match command.builtin_command {
            BuiltinCommand::ChangeDirectory(arguments) => {
                change_directory(&arguments, &mut stderror_writer)?
            }
            BuiltinCommand::Echo(command_string) => {
                echo(
                    command_string.as_slice(),
                    &mut stdout_writer,
                    &mut stderror_writer,
                )?;
            }
            BuiltinCommand::Exit => break,
            BuiltinCommand::PWD => pwd(&mut stdout_writer, &mut stderror_writer)?,
            BuiltinCommand::Type(arguments) => {
                builtin_type(arguments, &path, &mut stdout_writer, &mut stderror_writer)?;
            }
            BuiltinCommand::NotFound(command_string, arguments) => {
                if let Some(executable) = find_executable_file(&command_string, &path) {
                    run_external_executable(
                        executable,
                        &arguments,
                        &mut stdout_writer,
                        &mut stderror_writer,
                    )?;
                } else {
                    let error = CustomError::CommandNotFound(command_string);
                    stderror_writer
                        .send(format!("{error}"))
                        .context("Sending error to standard error.")?;
                }
            }
        }

        match command.standard_out {
            command::Output::Standard => {
                for message in stdout_reader.try_iter() {
                    print!("{}", message.trim());
                }
            }
            command::Output::File(input) => {
                let file_path = Path::new(&input);
                let mut file = std::fs::File::create(file_path)
                    .context("Creating standard out file as we're redirecting")?;

                for message in stdout_reader.try_iter() {
                    file.write_all(message.as_bytes())
                        .context("writing standard out to a file")?;
                }
            }
        }

        match command.standard_error {
            command::Output::Standard => {
                for message in stderror_reader.try_iter() {
                    eprint!("{}", message.trim());
                }
            }
            command::Output::File(_) => todo!(),
        }

        println!("");
    }

    Ok(())
}
