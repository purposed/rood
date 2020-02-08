use std::error::Error as IError;
use std::fmt;
use std::io;

#[derive(Debug, PartialEq)]
pub enum Cause {
    // Existence issues.
    AlreadyExists,
    NotFound,

    // Concurrency.
    ConcurrencyError,

    // User-related errors.
    InvalidData,
    InvalidState,
    SerializationError,

    // Application Errors
    IOError,

    GeneralError(String),
}

#[derive(Debug)]
pub struct Error {
    pub cause: Cause,
    pub message: String,
}

impl Error {
    pub fn new(cause: Cause, msg: &str) -> Error {
        Error {
            cause,
            message: String::from(msg),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: use fmt::Display for cause instead of std::Debug
        write!(f, "[{:?}] - {}", self.cause, self.message)
    }
}

impl IError for Error {}

impl From<io::Error> for Error {
    fn from(v: io::Error) -> Error {
        Error::new(Cause::IOError, &format!("{}", v))
    }
}

pub type CausedResult<T> = Result<T, Error>;
