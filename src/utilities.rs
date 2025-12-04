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
    let mut split_user_input = user_input.split_whitespace();
    let command_input = split_user_input.next().unwrap_or(" ").to_owned();
    let arguments = split_user_input
        .map(ToOwned::to_owned)
        .collect::<Vec<String>>();
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
