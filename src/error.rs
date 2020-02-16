use std::error::Error as IError;
use std::fmt;
use std::io;
use std::time;

/// Possible error causes. Useful for handling various failures from the code without having too much detail.
#[derive(Debug, PartialEq)]
pub enum Cause {
    AlreadyExists,
    NotFound,

    /// Error thrown when encountering an invalid state in a concurrent situation.
    /// *e.g.* when awaiting a stopped thread.
    ConcurrencyError,

    /// User-input related error. Return this when processing invalid user data.
    InvalidData,

    /// Return `InvalidState` when an operation fails due to a corrupt state. Recovery is often impossible.
    InvalidState,

    /// Return when encoutering issues serializing / deserializing data.
    SerializationError,

    /// Returned by `From<std::io::Error> for rood::Error`.
    IOError,

    /// Returned by `From<std::time::SystemTimeError> for rood::Error`.
    TimeError,

    /// Used to encapsulate errors from other crates and/or errors not well defined here.
    GeneralError(String),
}

/// `Error` is a general error type used for failures in components of the Rood library, as well
/// as by most purposed tools.
///
/// It can be converted implicitly from an `std::io::Error` and from a `std::Time::SystemTimeError`.
#[derive(Debug)]
pub struct Error {
    /// The cause of the error.
    pub cause: Cause,

    /// Additional information regarding the error.
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

impl From<time::SystemTimeError> for Error {
    fn from(v: time::SystemTimeError) -> Error {
        Error::new(Cause::TimeError, &format!("{}", v))
    }
}

/// `CausedResult<T>` is a type alias for `Result<T, Error>`.
///
/// It is mostly used to shorten function declarations across the purposed codebase.
pub type CausedResult<T> = Result<T, Error>;
