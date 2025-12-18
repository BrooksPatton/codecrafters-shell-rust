use anyhow::{Result, bail};
use std::{
    io::Read,
    process::{self, Stdio},
};

pub fn run_external(
    commands: Vec<(String, Vec<String>)>,
    stdout_redirect: Option<&mut Vec<String>>,
    stderr_redirect: Option<&mut Vec<String>>,
) -> Result<()> {
    let mut commands_iter = commands.iter().peekable();
    let mut cached_child_stderr = None;
    let mut cached_child_stdout = None;
    let mut cached_child = None;

    loop {
        let Some((command_name, arguments)) = commands_iter.next() else {
            break;
        };
        let mut command = process::Command::new(command_name);

        command.args(arguments);

        if stderr_redirect.is_some() {
            command.stderr(Stdio::piped());
        }

        if commands_iter.peek().is_some() || stdout_redirect.is_some() {
            command.stdout(Stdio::piped());
        }

        if let Some(previous_stdout) = cached_child_stdout.take() {
            command.stdin(Stdio::from(previous_stdout));
        }

        let mut child = command.spawn()?;

        if stderr_redirect.is_some() {
            cached_child_stderr = child.stderr.take();
        }

        if commands_iter.peek().is_some() || stdout_redirect.is_some() {
            cached_child_stdout = child.stdout.take();
        }

        cached_child = Some(child);
    }

    let Some(child) = cached_child else {
        bail!("no commands run");
    };

    if let Some(stderr) = stderr_redirect {
        if let Some(mut child_stderr) = cached_child_stderr {
            let mut error = String::new();

            child_stderr.read_to_string(&mut error)?;
            stderr.push(error);
        };
    };

    if let Some(stdout) = stdout_redirect {
        if let Some(mut child_stdout) = cached_child_stdout {
            let mut result = String::new();

            child_stdout.read_to_string(&mut result)?;
            stdout.push(result);
        };
    };

    let _output = child.wait_with_output()?;

    Ok(())
}
