use crate::{
    builtin_commands::{BuiltinCommand, CommandArguments, echo::echo},
    utilities::find_executable_file,
};
use std::path::PathBuf;

pub fn builtin_type(arguments: CommandArguments, paths: &[PathBuf]) {
    let type_input = arguments.first().cloned().unwrap_or_default();
    let builtin_command = BuiltinCommand::from(type_input.clone());
    let mut message = vec![];

    message.push(type_input.clone());

    if matches!(builtin_command, BuiltinCommand::NotFound(_, _)) {
        // search the path to see if we can find an executable
        if let Some(dir_entry) = find_executable_file(&type_input, paths) {
            let path_buf = dir_entry.path();
            let path = path_buf
                .into_os_string()
                .into_string()
                .unwrap_or("unknown path".to_owned());

            message.push(" is ".to_owned());
            message.push(path);
        } else {
            message.push(": not found".to_owned());
        };
    } else {
        message.push(" is a shell builtin".to_owned());
    }

    let message = message.join("");
    echo(&[&message]);
}
