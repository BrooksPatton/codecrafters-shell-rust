mod commands;
mod errors;
pub mod utilities;

#[allow(unused_imports)]
use std::io::{self, Write};

use anyhow::{Context, Result};

use crate::{
    commands::Command,
    errors::CustomError,
    utilities::{exit, get_command, print_error, print_prompt},
};

pub fn run() -> Result<()> {
    loop {
        print_prompt();

        let command = get_command().context("getting command")?;

        match command {
            Command::Exit => exit(0),
            Command::NotFound(command_string) => {
                let error = CustomError::CommandNotFound(command_string);
                print_error(error);
            }
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}
