use anyhow::Result;

use crate::{builtin_commands::BuiltinCommand, input_parser::parse_input};

#[derive(Debug)]
pub struct Command {
    pub builtin_command: BuiltinCommand,
    pub standard_out: Output,
    pub standard_error: Output,
    pub piped_commands: Vec<(String, Vec<String>)>,
}

impl Command {
    pub fn new(user_input: String, stderr_collector: &mut Vec<String>) -> Result<Self> {
        let mut parsed_input = parse_input(user_input);
        let command_input = parsed_input.remove(0);
        let (arguments, standard_out, standard_error) =
            Self::extract_redirect(parsed_input, stderr_collector)?;
        let (arguments, piped_commands) = Self::extract_pipe(arguments, stderr_collector)?;
        let builtin_command = BuiltinCommand::from((command_input, arguments.clone()));

        Ok(Self {
            builtin_command,
            standard_out,
            standard_error,
            piped_commands,
        })
    }

    fn extract_redirect(
        input: Vec<String>,
        stderr: &mut Vec<String>,
    ) -> Result<(Vec<String>, Output, Output)> {
        let mut arguments = vec![];
        let mut arguments_iter = input.into_iter();
        let mut standard_out_output = Output::Standard;
        let mut standard_error_output = Output::Standard;

        while let Some(argument) = arguments_iter.next() {
            match argument.as_str() {
                "1>" | ">" => {
                    let Some(next_argument) = arguments_iter.next() else {
                        stderr
                            .push("When redirecting standard out, a file must be given".to_owned());
                        break;
                    };
                    standard_out_output = Output::CreateFile(next_argument);
                }
                "1>>" | ">>" => {
                    let Some(next_argument) = arguments_iter.next() else {
                        stderr
                            .push("When redirecting standard out, a file must be given".to_owned());
                        break;
                    };
                    standard_out_output = Output::AppendFile(next_argument);
                }
                "2>" => {
                    let Some(next_argument) = arguments_iter.next() else {
                        stderr.push("When redirecting standard error to a file, you must provide a file name".to_owned());
                        break;
                    };

                    standard_error_output = Output::CreateFile(next_argument);
                }
                "2>>" => {
                    let Some(next_argument) = arguments_iter.next() else {
                        stderr.push("When redirecting standard error to a file, you must provide a file name".to_owned());
                        break;
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
    ) -> Result<(Vec<String>, Vec<(String, Vec<String>)>)> {
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
