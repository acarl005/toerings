//! Data collection for CPU usage and load average.
//!
//! For CPU usage, Linux, macOS, and Windows are handled by Heim, FreeBSD by sysinfo.
//!
//! For load average, macOS and Linux are supported through Heim, FreeBSD by sysinfo.

use serde::Serialize;

cfg_if::cfg_if! {
    if #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))] {
        pub mod heim;
        pub use self::heim::*;
    } else if #[cfg(target_os = "freebsd")] {
        pub mod sysinfo;
        pub use self::sysinfo::*;
    }
}

pub type LoadAvgHarvest = [f32; 3];

#[derive(Debug, Clone, Copy, Serialize)]
pub enum CpuDataType {
    Avg,
    Cpu(usize),
}

#[derive(Debug, Clone, Serialize)]
pub struct CpuData {
    pub data_type: CpuDataType,
    pub cpu_usage: f64,
}

pub type CpuHarvest = Vec<CpuData>;

pub type PastCpuWork = f64;
pub type PastCpuTotal = f64;

pub type Point = (f64, f64);
