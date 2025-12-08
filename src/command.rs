use std::fmt::Arguments;

use crate::{builtin_commands::BuiltinCommand, input_parser::parse_input, utilities::print_error};

#[derive(Debug)]
pub struct Command {
    pub builtin_command: BuiltinCommand,
    arguments: Vec<String>,
    pub standard_out: Output,
}

impl Command {
    pub fn new(user_input: String) -> Self {
        let mut parsed_input = parse_input(user_input);
        let command_input = parsed_input.remove(0);
        let (arguments, standard_out) = Self::extract_redirect(parsed_input);
        let builtin_command = BuiltinCommand::from((command_input, arguments.clone()));

        Self {
            builtin_command,
            arguments,
            standard_out,
        }
    }

    fn extract_redirect(input: Vec<String>) -> (Vec<String>, Output) {
        let mut arguments = vec![];
        let mut arguments_iter = input.into_iter();
        let mut output = Output::StandardOut;

        while let Some(argument) = arguments_iter.next() {
            match argument.as_str() {
                "1>" | ">" => {
                    let Some(next_argument) = arguments_iter.next() else {
                        print_error("When redirecting standard out, a file must be given");
                        break;
                    };
                    output = Output::File(next_argument);
                }
                _ => arguments.push(argument),
            }
        }

        (arguments, output)
    }
}

#[derive(Debug)]
pub enum Output {
    StandardOut,
    File(String),
}
