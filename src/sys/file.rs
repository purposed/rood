use std::fs;
use std::io;
use std::path::Path;

use std::io::Write;

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

use crate::{Cause, CausedResult, Error};

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

pub fn make_executable<T>(p: T) -> Result<(), io::Error> where T: AsRef<Path> {
    if cfg!(unix) {
        // TODO: Actually only add +x flag, nothing else.
        let mut perms = fs::metadata(p.as_ref())?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(p, perms)?;
    }
    Ok(())
}

pub fn replace_all<T>(p: T, pattern: &str, to: &str) -> CausedResult<()>
where
    T: AsRef<Path>,
{
    let data = fs::read_to_string(p.as_ref())?;
    fs::File::create(p.as_ref())?.write_all(data.replace(pattern, to).as_bytes())?;
    Ok(())
}
