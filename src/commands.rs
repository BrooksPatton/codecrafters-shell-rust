pub mod builtin_type;
pub mod echo;

pub type CommandArguments = Vec<String>;

pub enum Command {
    Echo(CommandArguments),
    Exit,
    Type(CommandArguments),
    NotFound(String),
}

impl From<(String, CommandArguments)> for Command {
    fn from((command, arguments): (String, CommandArguments)) -> Self {
        match command.as_str() {
            "echo" => Self::Echo(arguments),
            "exit" => Self::Exit,
            "type" => Self::Type(arguments),
            _ => Self::NotFound(command),
        }
    }
}

impl From<String> for Command {
    fn from(command: String) -> Self {
        let arguments = vec![];
        Self::from((command, arguments))
    }
}
