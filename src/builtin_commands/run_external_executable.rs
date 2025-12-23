use std::{
    io::{PipeReader, Read},
    num::NonZero,
    process::{self, Child, ChildStdout, Stdio},
};

use crate::{
    command::{Command, CommandIO},
    errors::ErrorExitCode,
};

pub fn run_external(
    command_name: String,
    arguments: Vec<String>,
    command_io: CommandIO,
    is_last_child: bool,
    stdin: Option<Stdio>,
    is_redirecting: bool,
) -> Result<Child, ErrorExitCode> {
    let mut command = process::Command::new(command_name);

    command.args(arguments);
    command.stderr(Stdio::from(command_io.stderr));
    command.env("COLORTERM", "truecolor");

    if let Some(stdio) = stdin {
        command.stdin(stdio);
    }

    if !is_last_child {
        command.stdout(Stdio::piped());
    } else if is_redirecting {
        command.stdout(Stdio::from(command_io.stdout));
    }

    let child = command.spawn()?;
    Ok(child)
}
