use anyhow::Result;

use crate::{builtin_commands::BuiltinCommand, utilities::find_executable_files};
use std::path::PathBuf;

pub fn builtin_type(
    arguments: Vec<String>,
    paths: &[PathBuf],
    stdout: &mut Vec<String>,
    stderr: &mut Vec<String>,
) -> Result<()> {
    let type_input = arguments.first().cloned().unwrap_or_default();
    let builtin_command = BuiltinCommand::from(type_input.clone());
    let mut message = vec![];
    let mut is_error = false;

    message.push(type_input.clone());

    if matches!(builtin_command, BuiltinCommand::NotFound(_, _)) {
        // search the path to see if we can find an executable
        let dir_entries = find_executable_files(&type_input, paths, false)?;
        if let Some(dir_entry) = dir_entries.first() {
            let path_buf = dir_entry.path();
            let path = path_buf
                .into_os_string()
                .into_string()
                .unwrap_or("unknown path".to_owned());

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
        stderr.push(message);
    } else {
        stdout.push(message);
    }

    Ok(())
}
