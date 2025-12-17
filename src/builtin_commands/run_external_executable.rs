use anyhow::Result;
use std::{
    fs::DirEntry,
    io::{BufRead, BufReader, BufWriter, Write},
    process::{self, Stdio},
    thread,
    time::Duration,
};

pub fn run_external_executable(
    executable: &DirEntry,
    arguments: Vec<String>,
    piped_commands: Vec<(String, Vec<String>)>,
    stdout: &mut Vec<String>,
    stderr: &mut Vec<String>,
) -> Result<()> {
    let command_name = executable.file_name();
    let command_name = command_name.to_str().unwrap();

    let mut first_command = process::Command::new(command_name)
        .args(arguments)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let Some(last_command_stdout) = first_command.stdout.take() else {
        stderr.push(format!("Command {command_name} doesn't have standard out"));
        return Ok(());
    };
    let mut last_command_stdout = BufReader::new(last_command_stdout);
    let mut last_command_buffer = last_command_stdout.fill_buf()?;
    let mut last_command_output = String::from_utf8(last_command_buffer.to_vec())?;

    thread::sleep(Duration::from_nanos(1));
    if let Some(_exit_code) = first_command.try_wait()? {
        if let Some(first_command_stderr) = first_command.stderr.take() {
            let mut stderr_reader = BufReader::new(first_command_stderr);
            let buffer = stderr_reader.fill_buf()?;
            let message = String::from_utf8(buffer.to_vec())?;

            stderr.push(message);
            return Ok(());
        };
    }

    for (piped_command_name, piped_command_arguments) in piped_commands {
        let mut piped_command = process::Command::new(piped_command_name.clone())
            .args(piped_command_arguments)
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        {
            let Some(piped_command_stdin) = piped_command.stdin.take() else {
                stderr.push("Error: piped command doesn't have a standard out. This is probably a shell problem".to_owned());
                return Ok(());
            };

            let mut piped_command_stdin_writer = BufWriter::new(piped_command_stdin);
            piped_command_stdin_writer.write_all(&mut last_command_output.as_bytes())?;
            piped_command_stdin_writer.flush()?;
        }

        loop {
            match piped_command.try_wait()? {
                Some(exit_status) => {
                    if exit_status.success() {
                        let Some(piped_command_stdout) = piped_command.stdout.take() else {
                            stderr
                                .push(format!("{piped_command_name} doesn't have a standard out"));
                            return Ok(());
                        };

                        last_command_stdout = BufReader::new(piped_command_stdout);
                        last_command_buffer = last_command_stdout.fill_buf()?;
                        last_command_output = String::from_utf8(last_command_buffer.to_vec())?;
                    } else {
                        if let Some(error) = piped_command.stderr.take() {
                            let mut reader = BufReader::new(error);
                            let bytes = reader.fill_buf()?;
                            let message = String::from_utf8(bytes.to_vec())?;

                            stderr.push(message);

                            return Ok(());
                        }
                    }
                    break;
                }
                None => {
                    piped_command.wait()?;
                }
            }
        }
    }

    let stdout_message = last_command_stdout.fill_buf()?;
    stdout.push(String::from_utf8(stdout_message.to_vec())?);

    Ok(())
}
