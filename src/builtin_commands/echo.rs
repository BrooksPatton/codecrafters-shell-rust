use crate::{command::CommandIO, errors::ErrorExitCode};
use std::io::Write;

pub fn echo(user_input: &[String], mut next_command_io: CommandIO) -> Result<(), ErrorExitCode> {
    let echo_string = user_input.join(" ");
    writeln!(next_command_io.stdout, "{echo_string}")?;

    Ok(())
}
