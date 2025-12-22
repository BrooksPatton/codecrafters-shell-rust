use crate::{
    builtin_commands::BuiltinCommand, command::CommandIO, errors::ErrorExitCode,
    utilities::find_executable_files,
};
use std::{io::Write, path::PathBuf};

pub fn builtin_type(
    arguments: Vec<String>,
    paths: &[PathBuf],
    mut command_io: CommandIO,
) -> Result<(), ErrorExitCode> {
    let type_input = arguments
        .first()
        .cloned()
        .ok_or(ErrorExitCode::new_const::<1>())?;
    let builtin_command = BuiltinCommand::from(type_input.clone());
    let mut message = vec![];
    let mut is_error = false;

    message.push(type_input.clone());

    if matches!(builtin_command, BuiltinCommand::NotFound(_, _)) {
        // search the path to see if we can find an executable
        let dir_entries = find_executable_files(&type_input, paths, false)
            .map_err(|_error| ErrorExitCode::new_const::<2>())?;
        if let Some(dir_entry) = dir_entries.first() {
            let path_buf = dir_entry.path();
            let path = path_buf.into_os_string().to_string_lossy().to_string();

            message.push(" is ".to_owned());
            message.push(path);
        } else {
            message.push(": not found".to_owned());
            is_error = true;
        };
    } else {
        message.push(" is a shell builtin".to_owned());
    }

    let message = message.join("");

    if is_error {
        writeln!(command_io.stderr, "{message}")
            .map_err(|_error| ErrorExitCode::new_const::<3>())?;
        Err(ErrorExitCode::new_const::<5>())
    } else {
        writeln!(command_io.stdout, "{message}").map_err(|_error| ErrorExitCode::new_const::<4>())
    }
}
