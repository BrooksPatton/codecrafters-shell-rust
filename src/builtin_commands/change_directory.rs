use crate::{command::CommandIO, errors::ErrorExitCode};
use std::{
    env::{home_dir, set_current_dir},
    io::Write,
    path::Path,
};

pub fn change_directory(
    arguments: &[String],
    mut command_io_in: CommandIO,
) -> Result<(), ErrorExitCode> {
    let Some(home_directory) = home_dir() else {
        writeln!(command_io_in.stderr, "Missing home directory")?;
        return Err(ErrorExitCode::new_const::<1>());
    };
    let target_path = match arguments.first() {
        Some(path) => {
            let path = Path::new(path);
            path.to_path_buf()
        }
        None => home_directory,
    };

    if target_path.is_dir() {
        if let Err(error) = set_current_dir(target_path) {
            writeln!(command_io_in.stderr, "{error:?}")?;
            return Err(ErrorExitCode::new_const::<2>());
        }
    } else {
        let target_path = target_path.to_string_lossy().into_owned();
        writeln!(
            command_io_in.stderr,
            "cd: {target_path}: No such file or directory"
        )?;
        return Err(ErrorExitCode::new_const::<3>());
    }

    Ok(())
}
