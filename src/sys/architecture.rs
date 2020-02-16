use std::fmt;

#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

/// Enum describing processor architectures.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serialization", serde(rename_all = "lowercase"))]
pub enum Architecture {
    // TODO: Support more architectures.
    Amd64,

    Unknown,
}

impl Architecture {
    // TODO: Add architecture detection.
    pub fn value(&self) -> String {
        match self {
            Architecture::Amd64 => String::from("amd64"),
            _ => String::from("unknown"),
        }
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl From<&str> for Architecture {
    fn from(v: &str) -> Architecture {
        match v {
            "amd64" => Architecture::Amd64,
            _ => Architecture::Unknown,
        }
    }
}
