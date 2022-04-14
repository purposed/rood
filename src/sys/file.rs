use std::fs;
use std::io;
use std::path::Path;

use std::io::Write;

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

/// Checks for existence of the provided path, returning an error in case it doesn't exist.
///
/// Mostly a convenient wrapper for spawning consistent [Error](../../error/struct.Error.html) instances
/// when checking for path existence.
///
/// # Examples
/// ```
/// use rood::sys::file;
/// let result = file::ensure_exists("/non_existent_file.txt");
/// assert!(result.is_err());
/// ```
pub fn ensure_exists<T>(raw: T) -> io::Result<()>
where
    T: AsRef<Path>,
{
    let path = raw.as_ref();

    if path.exists() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Path [{}] does not exist", path.display()),
        ))
    }
}

#[cfg(unix)]
fn make_exec_impl(path: &Path) -> io::Result<()> {
    // TODO: Actually only add +x flag, nothing else.
    let mut perms = fs::metadata(p.as_ref())?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(p, perms)?;
}

#[cfg(windows)]
fn make_exec_impl(_: &Path) -> io::Result<()> {
    Ok(())
}

/// Mark the specified file as executable.
///
/// On unix systems, the implementation will be equivalent to `chmod +x {p}`.
///
/// *Note: Does nothing for now on Windows.*
///
/// # Examples
/// ```
/// use rood::sys::file;
/// use std::fs;
///
/// let file_name = "myfile.sh";
/// let file_handle = fs::File::create(file_name).unwrap();
/// file::make_executable(file_name).unwrap();
/// # fs::remove_file(file_name);
/// ```
pub fn make_executable<T>(p: T) -> io::Result<()>
where
    T: AsRef<Path>,
{
    make_exec_impl(p.as_ref())
}

pub fn is_executable<T: AsRef<Path>>(_path: T) -> io::Result<bool> {
    #[cfg(unix)]
    {
        let perms = fs::metadata(_path.as_ref())?.permissions();
        return Ok(perms.mode() & 0o111 != 0);
    }

    return Ok(true);
}

/// Replace all instances of a pattern in file.
///
/// Will read the file into memory, and will replace every instance of the provided pattern before
/// writing back the buffer to the file.
///
/// # Examples
/// ```
/// use std::fs;
/// use rood::sys::file;
/// use std::io::Write;
///
/// let file_name = "myfile.txt";
///
/// // Create a file named "myfile.txt" and write some text to it.
/// fs::File::create(file_name).unwrap().write_all(b"Hello there world. Hello again.").unwrap();
///
/// // Use replace_all to replace all instances of "Hello" by "Goodbye".
/// file::replace_all(file_name, "Hello", "Goodbye").unwrap();
///
/// assert!(!fs::read_to_string(file_name).unwrap().contains("Hello"));
///
/// # fs::remove_file(file_name);
/// ```
pub fn replace_all<T>(p: T, pattern: &str, to: &str) -> io::Result<()>
where
    T: AsRef<Path>,
{
    let data = fs::read_to_string(p.as_ref())?;
    fs::File::create(p.as_ref())?.write_all(data.replace(pattern, to).as_bytes())?;
    Ok(())
}
