use std::{os::unix::fs::MetadataExt, path::PathBuf};

use crate::builtin_commands::{BuiltinCommand, CommandArguments, echo::echo};

pub fn builtin_type(arguments: CommandArguments, paths: &[PathBuf]) {
    let type_input = arguments.first().cloned().unwrap_or_default();
    let builtin_command = BuiltinCommand::from(type_input.clone());
    let mut message = vec![];

    message.push(type_input.clone());

    if matches!(builtin_command, BuiltinCommand::NotFound(_)) {
        // search the path to see if we can find an executable
        for path in paths {
            let Ok(dir_entries) = std::fs::read_dir(path) else {
                continue;
            };

            for dir_entry in dir_entries {
                let Ok(dir_entry) = dir_entry else {
                    continue;
                };

                if dir_entry.file_name().to_str().unwrap_or_default() != type_input {
                    continue;
                }

                let Ok(metadata) = dir_entry.metadata() else {
                    continue;
                };

                let mode = metadata.mode();
                let user_exec = mode & 0o100 != 0;
                let group_exec = mode & 0o010 != 0;
                let other_exec = mode & 0o001 != 0;

                if !user_exec && !group_exec && !other_exec {
                    continue;
                }

                let path_buf = dir_entry.path();
                let path = path_buf
                    .into_os_string()
                    .into_string()
                    .unwrap_or("unknown path".to_owned());

                message.push(" is ".to_owned());
                message.push(path);
                break;
            }

            if message.len() > 1 {
                break;
            }
        }
    } else {
        message.push(" is a shell builtin".to_owned());
    }

    if message.len() == 1 {
        message.push(": not found".to_owned());
    }

    let message = message.join("");
    echo(&[&message]);
}
