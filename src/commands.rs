pub enum Command {
    Exit,
    NotFound(String),
}

impl From<&str> for Command {
    fn from(user_input: &str) -> Self {
        match user_input {
            "exit" => Self::Exit,
            _ => Self::NotFound(user_input.to_owned()),
        }
    }
}
