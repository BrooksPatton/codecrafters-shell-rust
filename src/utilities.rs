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
    let (command_input, arguments) = parse_input(user_input);
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

enum ProcessArgumentsState {
    Command,
    InsideSingleQuotes,
    InsideDoubleQuotes,
    NotInQuotes,
}

impl ProcessArgumentsState {
    pub fn inside_quotes(&self) -> bool {
        matches!(self, Self::InsideSingleQuotes) || matches!(self, Self::InsideDoubleQuotes)
    }
}
/**
* Examples:
* input: 'hello      world'
* output: ["'hello      world'"]

*input: hello     world
* output: ["hello", "world"]
*/
fn parse_input(input: String) -> (String, Vec<String>) {
    let mut result = vec![];
    let mut current_argument = String::new();
    let mut state = ProcessArgumentsState::Command;
    let mut command_input = String::new();

    for argument_char in input.trim().chars() {
        if matches!(state, ProcessArgumentsState::Command) {
            if argument_char.is_whitespace() {
                state = ProcessArgumentsState::NotInQuotes;
                continue;
            } else {
                command_input.push(argument_char);
                continue;
            }
        }

        match argument_char {
            '\'' => {
                if matches!(state, ProcessArgumentsState::InsideSingleQuotes) {
                    state = ProcessArgumentsState::NotInQuotes;
                } else {
                    if matches!(state, ProcessArgumentsState::InsideDoubleQuotes) {
                        current_argument.push(argument_char);
                    } else {
                        state = ProcessArgumentsState::InsideSingleQuotes;
                    }
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
            _ => current_argument.push(argument_char),
        }
    }

    if !current_argument.is_empty() {
        result.push(current_argument);
    }

    (command_input, result)
}
