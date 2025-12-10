use std::{fmt::Display, io::Write, sync::mpsc::Sender};

use anyhow::{Context, Result};

pub fn echo(
    user_input: &[impl Display],
    output: &mut Sender<String>,
    standard_error: &mut Sender<String>,
) -> Result<()> {
    let mut inputs = user_input.iter();
    let mut buffer = vec![];
    if let Some(input) = inputs.next() {
        // print!("{input}");
        if let Err(error) = write!(&mut buffer, "{input}") {
            standard_error
                .send(format!("{error}"))
                .context("error sending message to stantard error.")?;
        };
    }

    for input in inputs {
        // print!(" {input}");
        if let Err(error) = write!(buffer, " {input}") {
            standard_error
                .send(format!("{error}"))
                .context("error sending message to standard error")?;
        };
    }

    output.send(String::from_utf8(buffer).unwrap()).unwrap();

    Ok(())
}
