use crate::{builtin_commands::BuiltinCommand, command::Command};
use anyhow::{Context, Result, bail};
pub use std::process::exit;
use std::{
    env::{self, split_paths},
    fs::DirEntry,
    io::{self, Write},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};

pub fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
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

pub fn find_files(name: &str, paths: &[PathBuf], partial_match: bool) -> Vec<DirEntry> {
    let mut found_entries = vec![];

    for path in paths {
        let Ok(directory) = std::fs::read_dir(path) else {
            continue;
        };

        for entry in directory {
            let Ok(entry) = entry else {
                continue;
            };
            let file_name = entry.file_name();

            if partial_match {
                if file_name
                    .to_str()
                    .is_some_and(|filename| filename.starts_with(name))
                {
                    found_entries.push(entry);
                }
            } else {
                if file_name == name {
                    found_entries.push(entry);
                }
            }
        }
    }

    found_entries
}

pub fn find_executable_files(
    name: &str,
    paths: &[PathBuf],
    partial_match: bool,
) -> Result<Vec<DirEntry>> {
    let dir_entries = find_files(name, paths, partial_match);
    let mut executable_files = vec![];

    for dir_entry in dir_entries {
        let metadata = dir_entry.metadata()?;
        let mode = metadata.mode();
        let user_exec = mode & 0o100 != 0;
        let group_exec = mode & 0o010 != 0;
        let other_exec = mode & 0o001 != 0;

        if user_exec || group_exec || other_exec {
            executable_files.push(dir_entry);
        }
    }

    Ok(executable_files)
}

pub fn write_all_to_file(buffer: &[u8], filename: &str) -> Result<()> {
    let file_path = Path::new(filename);
    let mut file = std::fs::File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)?;

    file.write_all(buffer)?;
    Ok(())
}

pub fn append_all_to_file(messages: &[String], filename: &str) -> Result<()> {
    let filtered_messages = messages
        .iter()
        .filter(|message| !message.is_empty())
        .collect::<Vec<&String>>();
    let file_path = Path::new(filename);
    let mut file = std::fs::File::options()
        .create(true)
        .append(true)
        .open(file_path)?;

    filtered_messages
        .iter()
        .map(|message| message.trim_end())
        .try_for_each(|message| file.write_all(message.as_bytes()))?;

    if !filtered_messages.is_empty() {
        file.write(b"\n")?;
    }

    Ok(())
}

pub fn find_matching_builtin(partial: &str) -> Result<Option<String>> {
    let matching_builtins = BuiltinCommand::matches(partial);

    Ok(if matching_builtins.len() > 1 {
        None
    } else if matching_builtins.len() == 1 {
        matching_builtins.first().cloned()
    } else {
        None
    })
}

/// This assumes that the word shares a common prefix with the prefix.
///
/// We return the number of remaining letters in the word after the prefix is removed
pub fn common_prefix_count(prefix: &str, word: &str) -> usize {
    word.get(prefix.len()..).unwrap_or_default().len()
}

pub fn all_matching_commands_lcp_the_same(matching_commands: &[(String, usize)]) -> bool {
    if matching_commands.len() == 1 {
        return false;
    }

    matching_commands[0].1 == matching_commands[matching_commands.len() - 1].1
}

pub fn are_all_items_same_length(list: &[String]) -> Result<bool> {
    if list.is_empty() {
        bail!("list must not be empty");
    }

    let length = list[0].len();

    for item in list {
        if item.len() != length {
            return Ok(false);
        }
    }

    Ok(true)
}

pub fn calculate_longest_common_prefix(word: &str, list: &[String]) -> Vec<(usize, String)> {
    let word_length = word.len();

    list.iter()
        .filter_map(|list_word| {
            let remaining_word = list_word.get(word_length..)?;
            let lcp = remaining_word.len();

            Some((lcp, list_word.to_owned()))
        })
        .collect()
}
