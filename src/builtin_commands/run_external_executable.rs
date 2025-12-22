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
    is_piping_stdout: bool,
    previous_child: Option<Child>,
) -> Result<Child, ErrorExitCode> {
    println!("about to run {command_name}");
    let mut command = process::Command::new(command_name);

    command.args(arguments);
    command.stderr(Stdio::from(command_io.stderr));
    command.env("COLORTERM", "truecolor");

    if is_piping_stdout {
        command.stdout(Stdio::piped());
    }

    if let Some(previous_child) = previous_child {
        println!("have a previous child");
        if let Some(stdout) = previous_child.stdout {
            println!("previous child has a stdout");
            command.stdin(Stdio::from(stdout));
        }
    }

    let child = command.spawn()?;
    println!("command spawned");
    Ok(child)
}
