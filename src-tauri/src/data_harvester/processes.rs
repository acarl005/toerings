//! Data collection for processes.
//!
//! For Linux, this is handled by a custom set of functions.
//! For Windows and macOS, this is handled by sysinfo.

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
        pub use self::linux::*;
    } else if #[cfg(target_os = "macos")] {
        pub mod macos;
        mod macos_freebsd;
        pub use self::macos::*;
    } else if #[cfg(target_os = "windows")] {
        pub mod windows;
        pub use self::windows::*;
    } else if #[cfg(target_os = "freebsd")] {
        pub mod freebsd;
        mod macos_freebsd;
        pub use self::freebsd::*;
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_family = "unix")] {
        pub mod unix;
        pub use self::unix::*;
    }
}

use serde::Serialize;

use crate::Pid;

#[derive(Debug, Clone, Default, Serialize)]
pub struct ProcessHarvest {
    /// The pid of the process.
    pub pid: Pid,

    /// The parent PID of the process. Remember, parent_pid 0 is root.
    pub parent_pid: Option<Pid>,

    /// CPU usage as a percentage.
    pub cpu_usage_percent: f64,

    /// Memory usage as a percentage.
    pub mem_usage_percent: f64,

    /// Memory usage as bytes.
    pub mem_usage_bytes: u64,

    /// The name of the process.
    pub name: String,

    /// The exact command for the process.
    pub command: String,

    /// Bytes read per second.
    pub read_bytes_per_sec: u64,

    /// Bytes written per second.
    pub write_bytes_per_sec: u64,

    /// The total number of bytes read by the process.
    pub total_read_bytes: u64,

    /// The total number of bytes written by the process.
    pub total_write_bytes: u64,

    /// The current state of the process (e.g. zombie, asleep)
    pub process_state: (String, char),

    /// This is the *effective* user ID of the process. This is only used on Unix platforms.
    #[cfg(target_family = "unix")]
    pub uid: Option<libc::uid_t>,

    /// This is the process' user. This is only used on Unix platforms.
    #[cfg(target_family = "unix")]
    pub user: std::borrow::Cow<'static, str>,
    // TODO: Additional fields
    // pub rss_kb: u64,
    // pub virt_kb: u64,
}
