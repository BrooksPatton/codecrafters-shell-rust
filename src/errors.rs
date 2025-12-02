use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("{0}: Command not found")]
    CommandNotFound(String),
}
