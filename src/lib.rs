mod builtin_commands;
mod command;
mod errors;
mod get_user_input;
pub mod input_parser;
pub mod utilities;

use crate::{
    builtin_commands::{
        BuiltinCommand, builtin_type::builtin_type, change_directory::change_directory, echo::echo,
        pwd::pwd, run_external_executable::run_external_executable,
    },
    errors::CustomError,
    get_user_input::UserInput,
    utilities::{
        append_all_to_file, find_executable_files, get_command, get_path, write_all_to_file,
    },
};
use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    let path = get_path().context("Getting path")?;
    let mut stdout: Vec<String> = vec![];
    let mut stderr: Vec<String> = vec![];
    let user_input = UserInput::new("$ ");

    loop {
        let user_input_line = user_input.readline()?;
        // if command is builtin: run it
        // if command is not builtin check if we can't find an executable print error
        // if command is executable run it

        let command = get_command(&mut stderr, user_input_line).context("getting command")?;

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
                if let Some(executable) =
                    find_executable_files(&command_string, &path, false)?.first()
                {
                    run_external_executable(
                        executable,
                        arguments,
                        command.piped_commands,
                        &mut stdout,
                        &mut stderr,
                    )?;
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
                    .map(|message| message.trim_end())
                    .for_each(|message| println!("{message}"));
            }
            command::Output::CreateFile(input) => {
                write_all_to_file(&stdout, &input).context("redirecting standard out to a file")?
            }
            command::Output::AppendFile(input) => append_all_to_file(&stdout, &input)
                .context("Error appending standard out to a file.")?,
        }

        match command.standard_error {
            command::Output::Standard => {
                stderr
                    .iter()
                    .map(|message| message.trim())
                    .for_each(|message| eprintln!("{message}"));
            }
            command::Output::CreateFile(input) => write_all_to_file(&stderr, &input)
                .context("redirecting standard error to a file")?,
            command::Output::AppendFile(input) => append_all_to_file(&stderr, &input)
                .context("Error appending standard error to a file.")?,
        }

        stderr.clear();
        stdout.clear();
    }

    Ok(())
}
