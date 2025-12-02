use std::{
    fmt::Display,
    io::{self, Write, stdin},
    process,
};

use anyhow::{Context, Result};

use crate::commands::Command;

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
    let command = Command::from(user_input.as_str());

    Ok(command)
}

pub fn exit(code: i32) {
    process::exit(code);
}
