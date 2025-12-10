use std::{fmt::Display, io::Write};

use anyhow::{Context, Result};

pub fn echo(
    user_input: &[impl Display],
    stdout: &mut Vec<String>,
    stderr: &mut Vec<String>,
) -> Result<()> {
    let mut inputs = user_input.iter();
    let mut buffer = vec![];
    if let Some(input) = inputs.next() {
        // print!("{input}");
        if let Err(error) = write!(&mut buffer, "{input}") {
            stderr.push(format!("{error}"));
        };
    }

    for input in inputs {
        // print!(" {input}");
        if let Err(error) = write!(buffer, " {input}") {
            stderr.push(format!("{error}"));
        };
    }

    stdout.push(String::from_utf8(buffer).context("Converting input from buffer to String")?);

    Ok(())
}
