use std::process::Command;

use crate::{Cause, CausedResult, Error};

#[cfg(unix)]
fn notify_impl(title: &str, message: &str) -> CausedResult<()> {
    let mut child_process = Command::new("notify-send")
        .arg(title)
        .arg(message)
        .spawn()?;
    let exit_status = child_process.wait()?;

    if !exit_status.success() {
        Err(Error::new(
            Cause::InvalidState,
            "Error executing notify-send",
        ))
    } else {
        Ok(())
    }
}

#[cfg(macos)]
fn notify_impl(title: &str, message: &str) -> CausedResult<()> {
    unimplemented!();
}

pub fn send(title: &str, message: &str) -> CausedResult<()> {
    notify_impl(title, message)
}
