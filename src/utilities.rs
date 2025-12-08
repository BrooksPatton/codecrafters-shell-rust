use crate::builtin_commands::BuiltinCommand;
use anyhow::{Context, Result, bail};
pub use std::process::exit;
use std::{
    env::{self, split_paths},
    fmt::Display,
    fs::DirEntry,
    io::{self, Write, stdin},
    os::unix::fs::MetadataExt,
    path::PathBuf,
};

pub fn get_user_input() -> Result<String> {
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .context("reading user input")?;
    Ok(user_input.trim().to_owned())
}

pub fn print_error(message: impl Display) {
    eprintln!("{message}",);
}

pub fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

pub fn get_command() -> Result<BuiltinCommand> {
    let user_input = get_user_input()?;
    let (command_input, argument_input) = extract_command_from_input(user_input);
    let arguments = parse_arguments(argument_input);
    let command = BuiltinCommand::from((command_input, arguments));

    Ok(command)
}

pub fn get_path() -> Result<Vec<PathBuf>> {
    let path = env::var("PATH").context("Getting PATH environment variable")?;
    let split_paths = split_paths(&path).map(|path| {
        if path.is_file() {
            bail!("PATH from environment variable is an file")
        } else {
            Ok(path)
        }
    });

    split_paths.collect()
}

pub fn find_files(name: &str, paths: &[PathBuf]) -> Vec<DirEntry> {
    paths
        .iter()
        .filter_map(|path| {
            let Ok(directory) = std::fs::read_dir(path) else {
                return None;
            };

            for dir_entry in directory {
                let Ok(dir_entry) = dir_entry else {
                    continue;
                };
                let file_name = dir_entry.file_name();

                if name == file_name {
                    return Some(dir_entry);
                } else {
                    continue;
                }
            }

            None
        })
        .collect()
}

pub fn find_executable_file(name: &str, paths: &[PathBuf]) -> Option<DirEntry> {
    let dir_entries = find_files(name, paths);

    for dir_entry in dir_entries {
        let metadata = dir_entry.metadata().ok()?;
        let mode = metadata.mode();
        let user_exec = mode & 0o100 != 0;
        let group_exec = mode & 0o010 != 0;
        let other_exec = mode & 0o001 != 0;

        if user_exec || group_exec || other_exec {
            return Some(dir_entry);
        }
    }

    None
}

fn extract_command_from_input(input: String) -> (String, String) {
    let mut command_input = String::new();
    let mut getting_command = true;
    let mut arguments = String::new();

    for argument_char in input.trim().chars() {
        if getting_command {
            if argument_char.is_whitespace() {
                getting_command = false;
            } else {
                command_input.push(argument_char);
            }
        } else {
            arguments.push(argument_char);
        }
    }

    (command_input, arguments)
}

enum ProcessArgumentsState {
    Escaping,
    InsideSingleQuotes,
    InsideDoubleQuotes,
    InsideDoubleQuotesEscaping,
    Normal,
}

impl ProcessArgumentsState {
    pub fn inside_quotes(&self) -> bool {
        matches!(self, Self::InsideSingleQuotes) || matches!(self, Self::InsideDoubleQuotes)
    }
}

fn parse_arguments(input: String) -> Vec<String> {
    let mut result = vec![];
    let mut current_argument = String::new();
    let mut state = ProcessArgumentsState::Normal;

    for argument_char in input.trim().chars() {
        match argument_char {
            '\'' => {
                if matches!(state, ProcessArgumentsState::InsideSingleQuotes) {
                    state = ProcessArgumentsState::Normal;
                } else if matches!(state, ProcessArgumentsState::InsideDoubleQuotesEscaping) {
                    current_argument.push('\\');
                    current_argument.push(argument_char);
                    state = ProcessArgumentsState::InsideDoubleQuotes;
                } else {
                    if matches!(state, ProcessArgumentsState::InsideDoubleQuotes) {
                        current_argument.push(argument_char);
                    } else {
                        state = ProcessArgumentsState::InsideSingleQuotes;
                    }
                }
            }
            '"' => {
                if matches!(state, ProcessArgumentsState::InsideDoubleQuotes) {
                    state = ProcessArgumentsState::Normal;
                } else if matches!(state, ProcessArgumentsState::InsideSingleQuotes) {
                    current_argument.push(argument_char);
                } else if matches!(state, ProcessArgumentsState::InsideDoubleQuotesEscaping) {
                    current_argument.push(argument_char);
                    state = ProcessArgumentsState::InsideDoubleQuotes;
                } else if matches!(state, ProcessArgumentsState::Escaping) {
                    current_argument.push(argument_char);
                    state = ProcessArgumentsState::Normal;
                } else {
                    state = ProcessArgumentsState::InsideDoubleQuotes;
                }
            }
            '~' => {
                if matches!(state, ProcessArgumentsState::InsideSingleQuotes) {
                    current_argument.push(argument_char);
                } else {
                    let home_directory = std::env::home_dir().unwrap_or_default();
                    current_argument.push_str(home_directory.to_str().unwrap_or_default());
                }
            }
            ' ' => {
                if state.inside_quotes() {
                    current_argument.push(argument_char);
                } else if !current_argument.is_empty() {
                    result.push(current_argument.clone());
                    current_argument.clear();
                }
            }
            '\\' => match state {
                ProcessArgumentsState::Escaping => {
                    current_argument.push(argument_char);
                    state = ProcessArgumentsState::Normal;
                }
                ProcessArgumentsState::InsideSingleQuotes => current_argument.push(argument_char),
                ProcessArgumentsState::InsideDoubleQuotes => {
                    state = ProcessArgumentsState::InsideDoubleQuotesEscaping
                }
                ProcessArgumentsState::InsideDoubleQuotesEscaping => {
                    current_argument.push(argument_char);
                    state = ProcessArgumentsState::InsideDoubleQuotes;
                }
                ProcessArgumentsState::Normal => state = ProcessArgumentsState::Escaping,
            },
            _ => {
                if matches!(state, ProcessArgumentsState::InsideDoubleQuotesEscaping) {
                    state = ProcessArgumentsState::InsideDoubleQuotes;
                    current_argument.push('\\');
                }
                current_argument.push(argument_char);
            }
        }
    }

    if !current_argument.is_empty() {
        result.push(current_argument);
    }

    result
}
