mod builtin_commands;
mod command;
mod errors;
mod get_user_input;
pub mod input_parser;
pub mod utilities;

use std::{
    env,
    io::{self, BufRead, BufReader, PipeReader, Write},
    process::{Child, Stdio},
};

use crate::{
    builtin_commands::{
        BuiltinCommand, builtin_type::builtin_type, change_directory::change_directory, echo::echo,
        pwd::pwd, run_external_executable::run_external,
    },
    command::{Command, CommandIO, parse_user_input},
    errors::ErrorExitCode,
    get_user_input::UserInput,
    utilities::{find_executable_files, get_path},
};
use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    let path = get_path().context("Getting path")?;
    let user_input = UserInput::new("$ ");

    'repl_loop: loop {
        let user_input_line = user_input.readline()?;
        let mut commands = match parse_user_input(user_input_line) {
            Ok(commands) => commands,
            Err(error) => {
                eprintln!("{error}");
                continue;
            }
        };
        // create the pipes here
        let mut previous_commands_stdout_reader: Option<PipeReader> = None;
        let mut previous_external_child: Option<Child> = None;
        let mut last_command: Option<Command> = None;

        while let Some(command) = commands.pop_front() {
            let current_command = Some(command.clone());

            let (mut stderr_reader, stderr_writer) = io::pipe()?;
            let (stdout_reader, stdout_writer) = io::pipe()?;
            let command_io_stdin = if let Some(unwrapped_last_command) = last_command.as_ref() {
                if unwrapped_last_command.builtin_command.is_builtin() {
                    Some(Stdio::from(previous_commands_stdout_reader.take().unwrap()))
                } else {
                    let Some(last_child) = previous_external_child.take() else {
                        unreachable!();
                    };
                    Some(Stdio::from(last_child.stdout.unwrap()))
                }
            } else {
                None
            };
            let mut next_command_io =
                CommandIO::new(command_io_stdin, stdout_writer, stderr_writer);
            let command_result = match command.builtin_command {
                BuiltinCommand::ChangeDirectory(arguments) => {
                    change_directory(&arguments, next_command_io)
                }
                BuiltinCommand::Echo(arguments) => echo(&arguments, next_command_io),
                BuiltinCommand::Exit => break 'repl_loop,
                BuiltinCommand::PWD => pwd(next_command_io),
                BuiltinCommand::Type(arguments) => builtin_type(arguments, &path, next_command_io),
                BuiltinCommand::NotFound(command_name, arguments) => {
                    if let Some(_executable) =
                        find_executable_files(&command_name, &path, false)?.first()
                    {
                        let mut child = run_external(
                            command_name,
                            arguments,
                            next_command_io,
                            commands.is_empty(),
                            !command.standard_out.is_standard(),
                        )?;

                        if commands.is_empty() {
                            let exited_child = child.wait()?;
                            if !exited_child.success() {
                                Err(ErrorExitCode::new(exited_child.code().unwrap()))
                            } else {
                                Ok(())
                            }
                        } else {
                            previous_external_child = Some(child);
                            Ok(())
                        }
                    } else {
                        writeln!(next_command_io.stderr, "{command_name}: Command not found")?;
                        drop(next_command_io.stderr);
                        Err(ErrorExitCode::new_const::<2>())
                    }
                }
            };

            match command_result {
                Ok(()) => {
                    let exit_code = 0;

                    previous_commands_stdout_reader = Some(stdout_reader);
                    unsafe {
                        env::set_var("?", exit_code.to_string());
                    }
                }
                Err(code) => {
                    unsafe { env::set_var("?", code.to_string()) }
                    match &current_command.as_ref().unwrap().standard_error {
                        command::Output::Standard => {
                            io::copy(&mut stderr_reader, &mut io::stderr())?;
                        }
                        command::Output::CreateFile(filename) => {
                            let mut reader = BufReader::new(stderr_reader);
                            let mut buffer = reader.fill_buf()?;
                            utilities::write_all_to_file(&mut buffer, filename)?;
                        }
                        command::Output::AppendFile(filename) => {
                            let buffer = BufReader::new(stderr_reader);
                            let lines = buffer
                                .lines()
                                .filter_map(|line| line.ok())
                                .collect::<Vec<String>>();
                            utilities::append_all_to_file(&lines, filename)?;
                        }
                    }
                }
            }

            last_command = current_command;
        }

        if let Some(mut stdout) = previous_commands_stdout_reader {
            if let Some(last_command) = last_command {
                match last_command.standard_out {
                    command::Output::Standard => {
                        io::copy(&mut stdout, &mut io::stdout())?;
                    }
                    command::Output::CreateFile(filename) => {
                        let mut reader = BufReader::new(stdout);
                        let mut buffer = reader.fill_buf()?;
                        utilities::write_all_to_file(&mut buffer, &filename)?;
                    }
                    command::Output::AppendFile(filename) => {
                        let buffer = BufReader::new(stdout);
                        let lines = buffer
                            .lines()
                            .filter_map(|line| line.ok())
                            .collect::<Vec<String>>();
                        utilities::append_all_to_file(&lines, &filename)?;
                    }
                }
            };
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

    Ok(())
}
