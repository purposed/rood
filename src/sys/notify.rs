use std::io;
use std::process::Command;

#[cfg(unix)]
fn notify_impl(title: &str, message: &str) -> io::Result<()> {
    let mut child_process = Command::new("notify-send")
        .arg(title)
        .arg(message)
        .spawn()?;
    let exit_status = child_process.wait()?;

    if !exit_status.success() {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Non-zero status code when calling notify-send",
        ))
    } else {
        Ok(())
    }
}

#[cfg(macos)]
fn notify_impl(title: &str, message: &str) -> io::Result<()> {
    unimplemented!();
}

pub fn send(title: &str, message: &str) -> io::Result<()> {
    notify_impl(title, message)
}
