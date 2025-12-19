use std::num::NonZero;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("{0}: command not found")]
    CommandNotFound(String),
    #[error("Error: missing filename")]
    FilenameMissing,
    #[error("Error: {0}")]
    Message(String),
    #[error("Error: {0} doesn't exist")]
    DirectoryOrFileNotFound(String),
}

/// Special thanks to Justus_Fluegel on Twitch for helping with errors
/// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=fb3dd1a5d6eec0899f16b45342652b80
pub struct ErrorExitCode(std::num::NonZero<i32>);

impl ErrorExitCode {
    pub fn new_const<const EXIT_CODE: i32>() -> Self {
        const { Self(NonZero::new(EXIT_CODE).unwrap()) }
    }
}

impl From<std::io::Error> for ErrorExitCode {
    fn from(_error: std::io::Error) -> Self {
        Self::new_const::<255>()
    }
}
