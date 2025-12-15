use anyhow::{Context, Result};
use std::{
    fs::DirEntry,
    process::{self, Stdio},
};

pub fn run_external_executable(
    executable: &DirEntry,
    arguments: Vec<String>,
    piped_command_names: Vec<(String, Vec<String>)>,
    stdout: &mut Vec<String>,
    stderr: &mut Vec<String>,
) -> Result<()> {
    let name = executable.file_name();
    let name = name.to_str().unwrap();
    let mut commands = vec![(process::Command::new(name), arguments)];
    commands.append(
        &mut piped_command_names
            .into_iter()
            .map(|(name, arguments)| (process::Command::new(name), arguments))
            .collect(),
    );

    let output = commands
        .into_iter()
        .try_fold(
            None::<process::Child>,
            |previous_child, (mut current_command, current_arguments)| {
                current_command.stdout(Stdio::piped());
                current_command.args(current_arguments);

                if let Some(mut child) = previous_child {
                    current_command.stdin(child.stdout.take().unwrap());
                } else {
                    current_command.stdin(Stdio::null());
                };

                let child = current_command.spawn()?;

                Ok::<_, anyhow::Error>(Some(child))
            },
        )?
        .unwrap()
        .wait_with_output()?;

    if !output.stdout.is_empty() {
        stdout.push(
            String::from_utf8(output.stdout)
                .context("Converting external command standard out to String")?,
        );
    }

    if !output.stderr.is_empty() {
        stderr.push(
            String::from_utf8(output.stderr)
                .context("Converting external command standard error to String")?,
        );
    }

    Ok(())
}
