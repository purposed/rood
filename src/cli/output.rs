use std::ops::Add;

use colored::*;

static STEP_PREFIX_MARKER: &str = "+";
static ERROR_PREFIX_MARKER: &str = "!";
static DEBUG_PREFIX_MARKER: &str = "-";
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

pub struct OutputManager {
    pub verbose: bool,
    padding: i32
}

impl OutputManager {
    pub fn new(verbose: bool) -> OutputManager {
        OutputManager{
            verbose,
            padding: 0
        }
    }

    pub fn push(&self) -> OutputManager {
        OutputManager{
            verbose: self.verbose,
            padding: self.padding+1
        }
    }

    pub fn with_padding(&self, padding: i32) -> OutputManager {
        OutputManager{
            verbose: self.verbose,
            padding
        }
    }

    fn print(&self, msg: ColoredString, prefix_marker: &str, verbose_only: bool) {
        if !self.verbose && verbose_only {
            return;
        }

        let pad = make_padding(self.padding);
        println!("{}{} {}", pad, prefix_marker, msg);
    }

    pub fn step(&self, msg: &str) {
        self.print(msg.yellow(), STEP_PREFIX_MARKER, false);
    }

    pub fn progress(&self, msg: &str) {
        self.print(ColoredString::from(msg), NO_PREFIX, false);
    }

    pub fn success(&self, msg: &str) {
        self.print(msg.green(), STEP_PREFIX_MARKER, false);
    }

    pub fn debug(&self, msg: &str) {
        self.print(msg.blue(), DEBUG_PREFIX_MARKER, true);
    }

    pub fn error(&self, msg: &str) {
        self.print(msg.red(), ERROR_PREFIX_MARKER, false);
    }

}

