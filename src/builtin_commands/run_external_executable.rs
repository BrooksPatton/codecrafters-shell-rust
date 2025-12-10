use std::{fs::DirEntry, sync::mpsc::Sender};

use anyhow::{Context, Result};

pub fn run_external_executable(
    executable: DirEntry,
    arguments: &[String],
    stdout: &mut Sender<String>,
    stderr: &mut Sender<String>,
) -> Result<()> {
    let name = executable.file_name();
    let name = name.to_str().unwrap();
    let mut command = std::process::Command::new(name);

    command.args(arguments);

    let command_result = command
        .output()
        .context("getting command result from external command")?;

    stdout
        .send(
            String::from_utf8(command_result.stdout)
                .context("Converting external command standard out to String")?,
        )
        .context("Sending external command standard out to Standard out channel")?;
    stderr
        .send(
            String::from_utf8(command_result.stderr)
                .context("Converting external command standard error to String")?,
        )
        .context("Sending external command standard error to Standard out channel")?;

    Ok(())
}
