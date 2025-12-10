use crate::command::Command;
use anyhow::{Context, Result, bail};
pub use std::process::exit;
use std::{
    env::{self, split_paths},
    fs::DirEntry,
    io::{self, Write, stdin},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};

pub fn get_user_input() -> Result<String> {
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .context("reading user input")?;
    Ok(user_input.trim().to_owned())
}

pub fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

pub fn get_command(standard_out: &mut Vec<String>) -> Result<Command> {
    let user_input = get_user_input()?;
    let command = Command::new(user_input, standard_out)?;

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

pub fn write_all_to_file(messages: &[String], filename: &str) -> Result<()> {
    let file_path = Path::new(filename);
    let mut file = std::fs::File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)?;

    messages
        .iter()
        .try_for_each(|message| file.write_all(message.as_bytes()))?;

    Ok(())
}

pub fn append_all_to_file(messages: &[String], filename: &str) -> Result<()> {
    let file_path = Path::new(filename);
    let mut file = std::fs::File::options()
        .create(true)
        .append(true)
        .open(file_path)?;

    // file.write(b"\r\n")
    //     .context("writing new line to appended file")?;

    messages
        .iter()
        .map(|message| message.trim())
        .try_for_each(|message| file.write_all(message.as_bytes()))?;

    Ok(())
}
