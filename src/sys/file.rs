use std::fs;
use std::io;
use std::path::Path;

use crate::{Cause, CausedResult, Error};

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

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

pub fn make_executable(p: &Path) -> Result<(), io::Error> {
    if cfg!(unix) {
        // TODO: Actually only add +x flag, nothing else.
        let mut perms = fs::metadata(p)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(p, perms)?;
    }
    Ok(())
}
