use std::ops::Add;

use colored::*;

static STEP_PREFIX_MARKER: &str = "+";
static ERROR_PREFIX_MARKER: &str = "!";
static DEBUG_PREFIX_MARKER: &str = "-";
static NO_PREFIX: &str = "";

fn make_padding(length: i32) -> String {
    let mut pad = String::new();

    if length > 0 {
        pad = String::from("|");
        for _ in 0..length {
            pad = pad.add(" ");
        }
    }
    pad
}

pub struct OutputManager {
    pub verbose: bool
}

impl OutputManager {
    pub fn new(verbose: bool) -> OutputManager {
        OutputManager{
            verbose
        }
    }

    fn print(&self, msg: ColoredString, prefix_marker: &str, padding: i32, verbose_only: bool) {
        if !self.verbose && verbose_only {
            return;
        }

        let pad = make_padding(padding);
        println!("{}{} {}", pad, prefix_marker, msg);
    }

    pub fn step(&self, msg: &str, padding: i32) {
        self.print(msg.yellow(), STEP_PREFIX_MARKER, padding, false);
    }

    pub fn progress(&self, msg: &str, padding: i32) {
        self.print(ColoredString::from(msg), NO_PREFIX, padding, false);
    }

    pub fn success(&self, msg: &str, padding: i32) {
        self.print(msg.green(), STEP_PREFIX_MARKER, padding, false);
    }

    pub fn debug(&self, msg: &str, padding: i32) {
        self.print(msg.blue(), DEBUG_PREFIX_MARKER, padding, true);
    }

    pub fn error(&self, msg: &str, padding: i32) {
        self.print(msg.red(), ERROR_PREFIX_MARKER, padding, false);
    }

}

