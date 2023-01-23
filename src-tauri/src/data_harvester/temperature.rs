//! Data collection for temperature metrics.
//!
//! For Linux and macOS, this is handled by Heim.
//! For Windows, this is handled by sysinfo.

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
        pub use self::linux::*;
    } else if #[cfg(any(target_os = "freebsd", target_os = "macos", target_os = "windows"))] {
        pub mod sysinfo;
        pub use self::sysinfo::*;
    }
}

#[cfg(feature = "nvidia")]
pub mod nvidia;

use serde::Serialize;

#[derive(Default, Debug, Clone, Serialize)]
pub struct TempHarvest {
    pub name: String,
    pub temperature: f32,
}
