use crate::commands::{Command, CommandArguments, echo::echo};

pub fn builtin_type(arguments: CommandArguments) {
    let command_input = arguments.first().cloned().unwrap_or_default();
    let command = Command::from(command_input.clone());
    let mut message = vec![];

    message.push(command_input);

    if matches!(command, Command::NotFound(_)) {
        message.push(format!(": not found"));
    } else {
        message.push(" is a shell builtin".to_owned());
    }

    let message = message.join("");
    echo(&[message])
}
