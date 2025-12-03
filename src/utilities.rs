use crate::builtin_commands::{BuiltinCommand, echo::echo};
use anyhow::{Context, Error, Result, anyhow, bail};
use core::error;
pub use std::process::exit;
use std::{
    env::{self, SplitPaths, split_paths},
    fmt::Display,
    io::{self, ErrorKind, Write, stdin},
    path::{Path, PathBuf},
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
