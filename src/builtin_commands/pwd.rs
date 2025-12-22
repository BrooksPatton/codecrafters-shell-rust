use crate::{command::CommandIO, errors::ErrorExitCode};
use std::io::Write;

pub fn pwd(mut stdout: CommandIO) -> Result<(), ErrorExitCode> {
    let path = std::env::current_dir().map_err(|_error| ErrorExitCode::new_const::<1>())?;
    let stringified_path = path.as_os_str().to_str().unwrap_or_default();

    writeln!(stdout.stdout, "{stringified_path}")
        .map_err(|_error| ErrorExitCode::new_const::<2>())?;

    Ok(())
}
