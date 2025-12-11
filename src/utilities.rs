use crate::{builtin_commands::BuiltinCommand, command::Command};
use anyhow::{Context, Result, bail};
use console::Term;
pub use std::process::exit;
use std::{
    env::{self, split_paths},
    fs::DirEntry,
    io::{self, Write},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};

pub fn get_user_input(term: &mut Term) -> Result<String> {
    let mut current_input = String::new();

    loop {
        let key = term
            .read_key()
            .context("reading key while getting user input")?;

        match key {
            console::Key::Unknown => todo!(),
            console::Key::UnknownEscSeq(_items) => todo!(),
            console::Key::ArrowLeft => todo!(),
            console::Key::ArrowRight => todo!(),
            console::Key::ArrowUp => todo!(),
            console::Key::ArrowDown => todo!(),
            console::Key::Enter => break,
            console::Key::Escape => todo!(),
            console::Key::Backspace => {
                current_input.pop();
                term.clear_line()?;
                print_prompt();
                print!("{current_input}");
            }
            console::Key::Home => todo!(),
            console::Key::End => todo!(),
            console::Key::Tab => {
                if let Some(command) = find_matching_command(&current_input)? {
                    current_input = command;
                    current_input.push(' ');
                    term.clear_line()?;
                    print_prompt();
                    print!("{current_input}");
                }
            }
            console::Key::BackTab => todo!(),
            console::Key::Alt => todo!(),
            console::Key::Del => todo!(),
            console::Key::Insert => todo!(),
            console::Key::PageUp => todo!(),
            console::Key::PageDown => todo!(),
            console::Key::Char(input_char) => {
                current_input.push(input_char);
                print!("{input_char}");
            }
            _ => todo!(),
        }
        std::io::stdout().flush()?;
    }

    println!();

    Ok(current_input)
}

pub fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

pub fn get_command(standard_out: &mut Vec<String>, term: &mut Term) -> Result<Command> {
    let user_input = get_user_input(term)?;
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
                    .is_some_and(|filename| filename.contains(name))
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

    if let Ok(metadata) = file.metadata() {
        if metadata.len() > 0 {
            file.write(b"\n")
                .context("writing new line to appended file")?;
        }
    } else {
        bail!("Cannot read open file for appending");
    }

    messages
        .iter()
        .map(|message| message.trim())
        .try_for_each(|message| file.write_all(message.as_bytes()))?;

    Ok(())
}

pub fn find_matching_command(partial: &str) -> Result<Option<String>> {
    let matching_builtins = BuiltinCommand::matches(partial);

    Ok(if matching_builtins.len() > 1 {
        None
    } else if matching_builtins.len() == 1 {
        matching_builtins.first().cloned()
    } else {
        let path = get_path()?;
        let possible_external_commands = find_executable_files(partial, &path, true)?;

        if possible_external_commands.len() == 1 {
            possible_external_commands[0].file_name().into_string().ok()
        } else {
            None
        }
    })
}
