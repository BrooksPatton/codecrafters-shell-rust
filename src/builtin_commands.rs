pub mod builtin_type;
pub mod change_directory;
pub mod echo;
pub mod pwd;
pub mod run_external_executable;

#[derive(Debug)]
pub enum BuiltinCommand {
    ChangeDirectory(Vec<String>),
    Echo(Vec<String>),
    Exit,
    PWD,
    Type(Vec<String>),
    NotFound(String, Vec<String>),
}

impl From<(String, Vec<String>)> for BuiltinCommand {
    fn from((command, arguments): (String, Vec<String>)) -> Self {
        match command.as_str() {
            "cd" => Self::ChangeDirectory(arguments),
            "echo" => Self::Echo(arguments),
            "exit" => Self::Exit,
            "pwd" => Self::PWD,
            "type" => Self::Type(arguments),
            _ => Self::NotFound(command.to_owned(), arguments),
        }
    }
}

impl From<String> for BuiltinCommand {
    fn from(command: String) -> Self {
        let arguments = vec![];
        Self::from((command, arguments))
    }
}
