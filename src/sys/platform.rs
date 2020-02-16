use std::fmt;

#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

/// Enum describing supported Purposed platforms.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serialization", serde(rename_all = "lowercase"))]
pub enum Platform {
    Windows,
    Darwin,
    Linux,
    Unknown,
}

impl Platform {
    /// Detect the current platform.
    ///
    /// Defaults to `Platform::Unknown` if the platform is unsupported by the Rood library.
    pub fn detect() -> Platform {
        if cfg!(windows) {
            Platform::Windows
        } else if cfg!(unix) {
            Platform::Linux
        } else if cfg!(macos) {
            Platform::Darwin
        } else {
            Platform::Unknown
        }
    }

    /// Returns the platform formatted as string.
    pub fn value(&self) -> String {
        match &self {
            Platform::Windows => String::from("windows"),
            Platform::Darwin => String::from("darwin"),
            Platform::Linux => String::from("linux"),
            Platform::Unknown => String::from("unknown"),
        }
    }
}
impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl From<&str> for Platform {
    fn from(v: &str) -> Platform {
        match v {
            "windows" => Platform::Windows,
            "darwin" => Platform::Darwin,
            "linux" => Platform::Linux,
            _ => Platform::Unknown,
        }
    }
}
