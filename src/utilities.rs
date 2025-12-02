use crate::commands::Command;
use anyhow::{Context, Result};
pub use std::process::exit;
use std::{
    fmt::Display,
    io::{self, Write, stdin},
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

pub fn get_command() -> Result<Command> {
    let user_input = get_user_input()?;
    let mut split_user_input = user_input.split_whitespace();
    let command_input = split_user_input.next().unwrap_or(" ").to_owned();
    let arguments = split_user_input
        .map(ToOwned::to_owned)
        .collect::<Vec<String>>();
    let command = Command::from((command_input, arguments));

    Ok(command)
}
