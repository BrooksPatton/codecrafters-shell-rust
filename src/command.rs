use std::sync::mpsc::Sender;

use anyhow::{Context, Result};

use crate::{builtin_commands::BuiltinCommand, input_parser::parse_input};

#[derive(Debug)]
pub struct Command {
    pub builtin_command: BuiltinCommand,
    pub standard_out: Output,
    pub standard_error: Output,
}

impl Command {
    pub fn new(user_input: String, standard_error_output: &mut Sender<String>) -> Result<Self> {
        let mut parsed_input = parse_input(user_input);
        let command_input = parsed_input.remove(0);
        let (arguments, standard_out) =
            Self::extract_redirect(parsed_input, standard_error_output)?;
        let builtin_command = BuiltinCommand::from((command_input, arguments.clone()));
        let standard_error = Output::Standard;

        Ok(Self {
            builtin_command,
            standard_out,
            standard_error,
        })
    }

    fn extract_redirect(
        input: Vec<String>,
        standard_error: &mut Sender<String>,
    ) -> Result<(Vec<String>, Output)> {
        let mut arguments = vec![];
        let mut arguments_iter = input.into_iter();
        let mut output = Output::Standard;

        while let Some(argument) = arguments_iter.next() {
            match argument.as_str() {
                "1>" | ">" => {
                    let Some(next_argument) = arguments_iter.next() else {
                        standard_error
                            .send("When redirecting standard out, a file must be given".to_owned())
                            .context("Sending error to standard error channel")?;
                        break;
                    };
                    output = Output::File(next_argument);
                }
                _ => arguments.push(argument),
            }
        }

        Ok((arguments, output))
    }
}

#[derive(Debug)]
pub enum Output {
    Standard,
    File(String),
}
