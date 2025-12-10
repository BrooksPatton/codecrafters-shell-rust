use anyhow::{Context, Result};
use std::fs::DirEntry;

pub fn run_external_executable(
    executable: DirEntry,
    arguments: &[String],
    stdout: &mut Vec<String>,
    stderr: &mut Vec<String>,
) -> Result<()> {
    let name = executable.file_name();
    let name = name.to_str().unwrap();
    let mut command = std::process::Command::new(name);

    command.args(arguments);

    let command_result = command
        .output()
        .context("getting command result from external command")?;

    if !command_result.stdout.is_empty() {
        stdout.push(
            String::from_utf8(command_result.stdout)
                .context("Converting external command standard out to String")?,
        );
    }

    if !command_result.stderr.is_empty() {
        stderr.push(
            String::from_utf8(command_result.stderr)
                .context("Converting external command standard error to String")?,
        );
    }

    Ok(())
}
