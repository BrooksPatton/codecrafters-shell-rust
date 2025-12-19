use crate::{
    builtin_commands::BuiltinCommand,
    errors::CustomError,
    input_parser::{input_for_one_command, parse_input},
};
use std::{
    collections::VecDeque,
    io::{self, PipeReader, PipeWriter},
};

#[derive(Debug)]
pub struct Command {
    pub builtin_command: BuiltinCommand,
    pub standard_out: Output,
    pub standard_error: Output,
}

impl Command {
    // pub fn new(user_input: String, stderr_collector: &mut Vec<String>) -> Result<Self> {
    //     let mut parsed_input = parse_input(user_input);
    //     let command_input = parsed_input.remove(0);
    //     let (arguments, standard_out, standard_error) =
    //         Self::extract_redirect(parsed_input, stderr_collector)?;
    //     let (arguments, piped_commands) = Self::extract_pipe(arguments, stderr_collector)?;
    //     let builtin_command = BuiltinCommand::from((command_input, arguments.clone()));

    //     Ok(Self {
    //         builtin_command,
    //         standard_out,
    //         standard_error,
    //     })
    // }
    pub fn new(mut user_input: VecDeque<String>) -> Result<Option<Self>, CustomError> {
        let Some(command_name) = user_input.pop_front() else {
            return Ok(None);
        };
        let (arguments, command_stdout, command_stderr) = Self::extract_redirect(user_input)?;
        let builtin_command = BuiltinCommand::from((command_name, arguments));

        Ok(Some(Self {
            builtin_command,
            standard_out: command_stdout,
            standard_error: command_stderr,
        }))
    }

    fn extract_redirect(
        input: VecDeque<String>,
    ) -> Result<(Vec<String>, Output, Output), CustomError> {
        let mut arguments = vec![];
        let mut arguments_iter = input.into_iter();
        let mut standard_out_output = Output::Standard;
        let mut standard_error_output = Output::Standard;

        while let Some(argument) = arguments_iter.next() {
            match argument.as_str() {
                "1>" | ">" => {
                    let Some(next_argument) = arguments_iter.next() else {
                        return Err(CustomError::FilenameMissing);
                    };
                    standard_out_output = Output::CreateFile(next_argument);
                }
                "1>>" | ">>" => {
                    let Some(next_argument) = arguments_iter.next() else {
                        return Err(CustomError::FilenameMissing);
                    };
                    standard_out_output = Output::AppendFile(next_argument);
                }
                "2>" => {
                    let Some(next_argument) = arguments_iter.next() else {
                        return Err(CustomError::FilenameMissing);
                    };

                    standard_error_output = Output::CreateFile(next_argument);
                }
                "2>>" => {
                    let Some(next_argument) = arguments_iter.next() else {
                        return Err(CustomError::FilenameMissing);
                    };

                    standard_error_output = Output::AppendFile(next_argument);
                }
                _ => arguments.push(argument),
            }
        }

        Ok((arguments, standard_out_output, standard_error_output))
    }

    fn extract_pipe(
        input: Vec<String>,
        stderr: &mut Vec<String>,
    ) -> Result<(Vec<String>, Vec<(String, Vec<String>)>), CustomError> {
        let mut arguments = vec![];
        let mut arguments_iter = input.into_iter().peekable();
        let mut piped_commands = vec![];

        while let Some(argument) = arguments_iter.next() {
            match argument.as_str() {
                "|" => {
                    let Some(command_name) = arguments_iter.next() else {
                        stderr.push("When piping output, a command must be given".to_owned());
                        break;
                    };
                    let mut pipe_arguments = vec![];

                    loop {
                        if arguments_iter.peek().is_some_and(|arg| arg != "|") {
                            pipe_arguments.push(arguments_iter.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    piped_commands.push((command_name, pipe_arguments));
                }
                _ => arguments.push(argument),
            }
        }

        Ok((arguments, piped_commands))
    }
}

#[derive(Debug)]
pub enum Output {
    Standard,
    CreateFile(String),
    AppendFile(String),
}

pub fn parse_user_input(user_input: String) -> Result<VecDeque<Command>, CustomError> {
    let mut commands = VecDeque::new();
    let mut parsed_input = parse_input(user_input);

    loop {
        let command_input = input_for_one_command(&mut parsed_input);
        let Some(command) = Command::new(command_input)? else {
            break;
        };

        commands.push_back(command);
    }

    Ok(commands)
}

/// Thanks to Justus_Flegel for help with pipes and this pattern.
/// we've only partially implemented it so far
/// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=99c818e83dfaa1204dc44cca93498bc1
pub struct CommandIO {
    pub stdin: PipeReader,
    pub stdout: PipeWriter,
    pub stderr: PipeWriter,
}

impl CommandIO {
    pub fn new(stdin: PipeReader, stdout: PipeWriter, stderr: PipeWriter) -> Self {
        Self {
            stdin,
            stdout,
            stderr,
        }
    }
}
