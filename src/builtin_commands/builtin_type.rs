use crate::{
    builtin_commands::{BuiltinCommand, CommandArguments, echo::echo},
    utilities::{exit, find_file},
};
use std::{os::unix::fs::MetadataExt, path::PathBuf};

pub fn builtin_type(arguments: CommandArguments, paths: &[PathBuf]) {
    let type_input = arguments.first().cloned().unwrap_or_default();
    let builtin_command = BuiltinCommand::from(type_input.clone());
    let mut message = vec![];

    message.push(type_input.clone());

    if matches!(builtin_command, BuiltinCommand::NotFound(_)) {
        // search the path to see if we can find an executable
        if let Some(dir_entry) = find_file(&type_input, paths) {
            let Ok(metadata) = dir_entry.metadata() else {
                echo(&["Error", "cannot read file metadata"]);
                exit(1);
            };
            let mode = metadata.mode();
            let user_exec = mode & 0o100 != 0;
            let group_exec = mode & 0o010 != 0;
            let other_exec = mode & 0o001 != 0;

            if user_exec && group_exec && other_exec {
                let path_buf = dir_entry.path();
                let path = path_buf
                    .into_os_string()
                    .into_string()
                    .unwrap_or("unknown path".to_owned());

                message.push(" is ".to_owned());
                message.push(path);
            }
        } else {
            message.push(": not found".to_owned());
        };
    } else {
        message.push(" is a shell builtin".to_owned());
    }

    let message = message.join("");
    echo(&[&message]);
}
