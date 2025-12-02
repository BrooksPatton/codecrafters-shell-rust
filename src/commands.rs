pub mod echo;

pub enum Command {
    Echo(Vec<String>),
    Exit,
    NotFound(String),
}

impl From<(String, Vec<String>)> for Command {
    fn from((command, arguments): (String, Vec<String>)) -> Self {
        match command.as_str() {
            "echo" => Self::Echo(arguments),
            "exit" => Self::Exit,
            _ => Self::NotFound(command),
        }
    }
}
