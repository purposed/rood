use std::io::{self, stdin, stdout, Write};
use std::ops::Add;
use std::process::Command;

use atty::Stream;

use colored::*;

static STEP_PREFIX_MARKER: &str = "+";
static ERROR_PREFIX_MARKER: &str = "!";
static DEBUG_PREFIX_MARKER: &str = "-";
static QUESTION_PREFIX_MARKER: &str = "?";
static NO_PREFIX: &str = "";

fn make_padding(length: i32) -> String {
    let mut pad = String::new();

    if length > 0 {
        pad = String::new();
        for _ in 0..length {
            pad = pad.add("|   ");
        }
    }
    pad
}

/// The OutputManager is a nice wrapper over shell i/o functions.
///
/// It provides output nesting, various log levels and verbosity levels, as well as stack-like features
/// for controlling more easily the indentation level of the output
///
/// # Examples
/// ## Simple usage and stacked usage.
/// ```
/// use rood::cli::OutputManager;
///
/// let verbose_mode = false;
/// let output = OutputManager::new(verbose_mode);
/// output.step("[Step 1]");
/// output.push().debug("Some indented verbose-only message");
/// output.success("[Step 1] - OK")
/// ```
///
/// ## Using a Yes/No prompt and handling errors.
/// ```
/// use rood::cli::OutputManager;
///
/// let output = OutputManager::new(false);
/// match output.prompt_yn("Really to quit?", false) {
///     Ok(val) => output.success(&format!("User picked: {}", val)),
///     Err(e) => output.error(&format!("Error: {}", e))
/// }
/// ```
#[derive(Clone)]
pub struct OutputManager {
    pub verbose: bool,
    padding: i32,
}

impl OutputManager {
    /// Returns an `OutputManager` of the specified verbosity.
    pub fn new(verbose: bool) -> OutputManager {
        OutputManager {
            verbose,
            padding: 0,
        }
    }

    /// Returns a **new** `OutputManager` with the same verbosity level, but with an indentation
    /// level incremented by one.
    pub fn push(&self) -> OutputManager {
        OutputManager {
            verbose: self.verbose,
            padding: self.padding + 1,
        }
    }

    /// Returns a **new** `OutputManager` with the same verbosity level, but with an indentation
    /// level set to the level specified.
    pub fn with_padding(&self, padding: i32) -> OutputManager {
        OutputManager {
            verbose: self.verbose,
            padding,
        }
    }

    fn print(&self, msg: ColoredString, prefix_marker: &str, verbose_only: bool) {
        if !self.verbose && verbose_only {
            return;
        }

        if !atty::is(Stream::Stdout) {
            let v: &str = &msg;
            println!("{}", v);
        } else {
            let pad = make_padding(self.padding);
            println!("{}{} {}", pad, prefix_marker, msg);
        }
    }

    fn print_sameline(&self, msg: ColoredString, prefix_marker: &str, verbose_only: bool) {
        if !self.verbose && verbose_only {
            return;
        }

        if !atty::is(Stream::Stdout) {
            let v: &str = &msg;
            print!("{}", v);
        } else {
            let pad = make_padding(self.padding);
            print!("{}{} {}", pad, prefix_marker, msg);
        }
    }

    /// Displays a step. `msg` will be printed in yellow, prefixed by a `+`.
    pub fn step<S: AsRef<str>>(&self, msg: S) {
        self.print(msg.as_ref().yellow(), STEP_PREFIX_MARKER, false);
    }

    /// Displays a progress message. ̀msg` will be printed in white without a prefix.
    pub fn progress<S: AsRef<str>>(&self, msg: S) {
        self.print(ColoredString::from(msg.as_ref()), NO_PREFIX, false);
    }

    /// Displays a success message. `msg` will be printed in green, prefixed by a '+'.
    pub fn success<S: AsRef<str>>(&self, msg: S) {
        self.print(msg.as_ref().green(), STEP_PREFIX_MARKER, false);
    }

    /// Displays a debug message. `msg` will be printed in blue, prefixed by a '-'.
    /// *Note: Debug messages are only printed when the verbose flag is set.*
    pub fn debug<S: AsRef<str>>(&self, msg: S) {
        self.print(msg.as_ref().blue(), DEBUG_PREFIX_MARKER, true);
    }

    /// Displays an error message. `msg` will be printed in red, prefixed by a '!'.
    pub fn error<S: AsRef<str>>(&self, msg: S) {
        self.print(msg.as_ref().red(), ERROR_PREFIX_MARKER, false);
    }

    /// Displays a prompt. `msg` will be printed in blue, prefixed by a '?'.
    /// Will wait for user input before returning what the user typed.
    pub fn prompt<S: AsRef<str>>(&self, msg: S) -> io::Result<String> {
        self.print_sameline(msg.as_ref().blue(), QUESTION_PREFIX_MARKER, false);
        let mut user_input = String::new();
        stdout().flush()?;
        stdin().read_line(&mut user_input)?;

        Ok(String::from(user_input.trim()))
    }

    /// Displays a yes/no prompt. `msg` will be printed in blue, prefixed by a '?'.
    /// Will wait for user input before returning what the user selected.
    pub fn prompt_yn<S: AsRef<str>>(&self, msg: S, default: bool) -> io::Result<bool> {
        if default {
            self.print_sameline(
                format!("{} [Y/n] - ", msg.as_ref()).blue(),
                QUESTION_PREFIX_MARKER,
                false,
            );
        } else {
            self.print_sameline(
                format!("{} [y/N] - ", msg.as_ref()).blue(),
                QUESTION_PREFIX_MARKER,
                false,
            );
        }
        let mut user_input = String::new();
        stdout().flush()?;
        stdin().read_line(&mut user_input)?;

        let user_pick = user_input.trim().to_ascii_lowercase();
        if default {
            Ok(&user_pick != "n")
        } else {
            Ok(&user_pick == "y")
        }
    }

    /// Displays a prompt for a password or any sensitive information. `msg` will be printed in blue, prefixed by a '?'.
    /// Will wait for user input before returning what the user typed.
    pub fn prompt_password<S: AsRef<str>>(&self, msg: S) -> io::Result<String> {
        self.print_sameline(msg.as_ref().blue(), QUESTION_PREFIX_MARKER, false);
        stdout().flush()?;
        let user_input = rpassword::read_password()?;

        Ok(String::from(user_input))
    }

    pub fn clear(&self) -> io::Result<()> {
        if cfg!(unix) {
            if !Command::new("clear").status()?.success() {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to clear the terminal",
                ))
            } else {
                Ok(())
            }
        } else if cfg!(windows) {
            if !Command::new("cls").status()?.success() {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to clear the terminal",
                ))
            } else {
                Ok(())
            }
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Unsupported platform"))
        }
    }
}
