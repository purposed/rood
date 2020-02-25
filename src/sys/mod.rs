mod architecture;

/// File-related utilities.
///
/// Most utilities defined in this module are higher-level abstractions over `std::fs`, but common
/// operations nonetheless.
pub mod file;

pub mod notify;

mod platform;

pub use architecture::Architecture;
pub use platform::Platform;
