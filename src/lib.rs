/// Useful functionality for command-line interfaces.
///
/// Features CLI prompts, colored & stacked outputs.
#[cfg(feature = "cli")]
pub mod cli;

/// Regroupment of system utilities, filesystem stuff, and portable wrappers around platform-specific
/// functionality.
#[cfg(feature = "sys")]
pub mod sys;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
