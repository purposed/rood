#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "errors")]
pub mod error;
#[cfg(feature = "errors")]
pub use error::{Cause, CausedResult, Error};

#[cfg(feature = "sys")]
pub mod sys;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
