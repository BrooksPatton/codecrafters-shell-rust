pub mod builtin_type;
pub mod echo;
pub mod pwd;
pub mod run_external_executable;

pub type CommandArguments = Vec<String>;

pub enum BuiltinCommand {
    Echo(CommandArguments),
    Exit,
    PWD,
    Type(CommandArguments),
    NotFound(String, CommandArguments),
}

impl From<(String, CommandArguments)> for BuiltinCommand {
    fn from((command, arguments): (String, CommandArguments)) -> Self {
        match command.as_str() {
            "echo" => Self::Echo(arguments),
            "exit" => Self::Exit,
            "pwd" => Self::PWD,
            "type" => Self::Type(arguments),
            _ => Self::NotFound(command, arguments),
        }
    }
}

impl From<String> for BuiltinCommand {
    fn from(command: String) -> Self {
        let arguments = vec![];
        Self::from((command, arguments))
    }
}
