use std::{fmt, io};

#[derive(Debug)]
pub enum ShellError {
    CommandNotFound(String),
    InvalidArgs { command: String, message: String },
    IOError(io::Error),
}
use ShellError::*;

impl ShellError {
    pub fn command_not_found(command: &str) -> Self {
        CommandNotFound(command.to_owned())
    }

    pub fn invalid_args(command: &str, message: &str) -> Self {
        InvalidArgs {
            command: command.to_owned(),
            message: message.to_owned(),
        }
    }
}
impl From<io::Error> for ShellError {
    fn from(e: io::Error) -> Self {
        IOError(e)
    }
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandNotFound(command) => writeln!(f, "{}: command not found", command),
            InvalidArgs { command, message } => writeln!(f, "{}: {}", command, message),
            IOError(e) => writeln!(f, "{}", e),
        }
    }
}

impl std::error::Error for ShellError {}

pub type ShellResult<T> = Result<T, ShellError>;
