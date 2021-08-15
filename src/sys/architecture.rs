use std::env::consts;
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
    Arm,
    Arm64,

    Unknown,
}

impl Architecture {
    pub fn detect() -> Architecture {
        match consts::ARCH {
            "x86_64" => Architecture::Amd64,
            "arm" => Architecture::Arm,
            "aarch64" => Architecture::Arm64,
            _ => Architecture::Unknown,
        }
    }

    pub fn value(&self) -> Vec<String> {
        match self {
            Architecture::Amd64 => vec![
                String::from("amd64"),
                String::from("x64"),
                String::from("x86_64"),
            ],
            Architecture::Arm => vec![String::from("arm")],
            &Architecture::Arm64 => vec![String::from("aarch64"), String::from("arm64")],
            _ => vec![String::from("unknown")],
        }
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value().first().unwrap())
    }
}

impl From<&str> for Architecture {
    fn from(v: &str) -> Architecture {
        match v {
            "amd64" => Architecture::Amd64,
            "arm" => Architecture::Arm,
            "aarch64" => Architecture::Arm64,
            _ => Architecture::Unknown,
        }
    }
}
