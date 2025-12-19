mod builtin_commands;
mod command;
mod errors;
mod get_user_input;
pub mod input_parser;
pub mod utilities;

use std::{
    fs::exists,
    io::{self, PipeReader},
};

use crate::{
    builtin_commands::{
        BuiltinCommand, builtin_type::builtin_type, change_directory::change_directory, echo::echo,
        pwd::pwd, run_external_executable::run_external,
    },
    command::{CommandIO, parse_user_input},
    errors::CustomError,
    get_user_input::UserInput,
    utilities::{find_executable_files, get_path},
};
use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    let path = get_path().context("Getting path")?;
    let user_input = UserInput::new("$ ");

    loop {
        let user_input_line = user_input.readline()?;
        let mut commands = match parse_user_input(user_input_line) {
            Ok(commands) => commands,
            Err(error) => {
                eprintln!("{error}");
                continue;
            }
        };
        // create the pipes here
        let mut previous_commands_std_reader: Option<PipeReader> = None;

        while let Some(command) = commands.pop_front() {
            let piping_command_stdout = !commands.is_empty();
            let (stdin_reader, stdin_writer) = io::pipe()?;
            let (mut stderr_reader, stderr_writer) = io::pipe()?;
            let (stdout_reader, stdout_writer) = io::pipe()?;

            previous_commands_std_reader = Some(stdout_reader);
            let next_command_io = CommandIO::new(stdin_reader, stdout_writer, stderr_writer);

            let command_result = match command.builtin_command {
                BuiltinCommand::ChangeDirectory(arguments) => {
                    change_directory(&arguments, next_command_io)
                }
                BuiltinCommand::Echo(arguments) => echo(&arguments, next_command_io),
                // BuiltinCommand::Exit => break,
                // BuiltinCommand::PWD => pwd(&mut stdout, &mut errors)?,
                // BuiltinCommand::Type(arguments) => {
                //     builtin_type(arguments, &path, &mut stdout, &mut errors)?
                // }
                // BuiltinCommand::NotFound(command_name, arguments) => {
                //     if let Some(executable) =
                //         find_executable_files(&command_name, &path, false)?.first()
                //     {
                //         let first_command_name = executable.file_name();
                //         let first_command_name =
                //             first_command_name.to_str().unwrap_or_default().to_owned();
                //         let mut commands = vec![(first_command_name, arguments)];
                //         let stdout = if matches!(command.standard_out, command::Output::Standard) {
                //             None
                //         } else {
                //             Some(&mut stdout)
                //         };
                //         let stderr = if matches!(command.standard_error, command::Output::Standard)
                //         {
                //             None
                //         } else {
                //             Some(&mut errors)
                //         };

                //         commands.append(&mut command.piped_commands);
                //         run_external(commands, stdout, stderr)?;
                //     } else {
                //         let error = CustomError::CommandNotFound(command_string);
                //         errors.push(format!("{error}"));
                //     }
                // }
                _ => todo!(),
            };

            match command_result {
                Ok(()) => (),
                Err(_code) => {
                    io::copy(&mut stderr_reader, &mut io::stderr())?;
                }
            }
        }

        if let Some(mut stdout) = previous_commands_std_reader {
            io::copy(&mut stdout, &mut io::stdout())?;
        }

        //     match command.builtin_command {
        //         BuiltinCommand::ChangeDirectory(arguments) => {
        //             change_directory(&arguments, &mut stderr)?
        //         }
        //         BuiltinCommand::Echo(command_string) => {
        //             echo(command_string.as_slice(), &mut stdout, &mut stderr)?;
        //         }
        //         BuiltinCommand::Exit => break,
        //         BuiltinCommand::PWD => pwd(&mut stdout, &mut stderr)?,
        //         BuiltinCommand::Type(arguments) => {
        //             builtin_type(arguments, &path, &mut stdout, &mut stderr)?;
        //         }
        //         BuiltinCommand::NotFound(command_string, arguments) => {
        //             if let Some(executable) =
        //                 find_executable_files(&command_string, &path, false)?.first()
        //             {
        //                 let first_command_name = executable.file_name();
        //                 let first_command_name =
        //                     first_command_name.to_str().unwrap_or_default().to_owned();
        //                 let mut commands = vec![(first_command_name, arguments)];
        //                 let stdout = if matches!(command.standard_out, command::Output::Standard) {
        //                     None
        //                 } else {
        //                     Some(&mut stdout)
        //                 };
        //                 let stderr = if matches!(command.standard_error, command::Output::Standard) {
        //                     None
        //                 } else {
        //                     Some(&mut stderr)
        //                 };

        //                 commands.append(&mut command.piped_commands);
        //                 run_external(commands, stdout, stderr)?;
        //             } else {
        //                 let error = CustomError::CommandNotFound(command_string);
        //                 stderr.push(format!("{error}"));
        //             }
        //         }
        //     }

        //     match command.standard_out {
        //         command::Output::Standard => {
        //             stdout
        //                 .iter()
        //                 .map(|message| message.trim_end())
        //                 .for_each(|message| println!("{message}"));
        //         }
        //         command::Output::CreateFile(input) => {
        //             write_all_to_file(&stdout, &input).context("redirecting standard out to a file")?
        //         }
        //         command::Output::AppendFile(input) => append_all_to_file(&stdout, &input)
        //             .context("Error appending standard out to a file.")?,
        //     }

        //     match command.standard_error {
        //         command::Output::Standard => {
        //             stderr
        //                 .iter()
        //                 .map(|message| message.trim())
        //                 .for_each(|message| eprintln!("{message}"));
        //         }
        //         command::Output::CreateFile(input) => write_all_to_file(&stderr, &input)
        //             .context("redirecting standard error to a file")?,
        //         command::Output::AppendFile(input) => append_all_to_file(&stderr, &input)
        //             .context("Error appending standard error to a file.")?,
        //     }

        //     stderr.clear();
        //     stdout.clear();
    }
}
