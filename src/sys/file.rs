use std::fs;
use std::io;
use std::path::Path;

use std::io::Write;

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

use crate::{Cause, CausedResult, Error};

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
pub fn ensure_exists<T>(raw: T) -> CausedResult<()>
where
    T: AsRef<Path>,
{
    let path = raw.as_ref();

    if path.exists() {
        Ok(())
    } else {
        Err(Error::new(
            Cause::NotFound,
            &format!("Path [{}] does not exist", path.to_str().unwrap_or("")),
        ))
    }
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
/// use std::fs::File;
///
/// let file_name = "myfile.sh";
/// let file_handle = File::create(file_name).unwrap();
/// file::make_executable(file_name).unwrap();
/// ```
pub fn make_executable<T>(p: T) -> Result<(), io::Error>
where
    T: AsRef<Path>,
{
    if cfg!(unix) {
        // TODO: Actually only add +x flag, nothing else.
        let mut perms = fs::metadata(p.as_ref())?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(p, perms)?;
    }
    Ok(())
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
/// ```
pub fn replace_all<T>(p: T, pattern: &str, to: &str) -> CausedResult<()>
where
    T: AsRef<Path>,
{
    let data = fs::read_to_string(p.as_ref())?;
    fs::File::create(p.as_ref())?.write_all(data.replace(pattern, to).as_bytes())?;
    Ok(())
}
