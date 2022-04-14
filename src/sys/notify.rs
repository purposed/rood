use std::io;

#[cfg(target_os = "linux")]
fn notify_impl(title: &str, message: &str) -> io::Result<()> {
    use std::process::Command;

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

#[cfg(target_os = "macos")]
fn notify_impl(_title: &str, _message: &str) -> io::Result<()> {
    unimplemented!();
}

#[cfg(target_os = "windows")]
fn notify_impl(_title: &str, _message: &str) -> io::Result<()> {
    unimplemented!();
}

pub fn send(title: &str, message: &str) -> io::Result<()> {
    notify_impl(title, message)
}
