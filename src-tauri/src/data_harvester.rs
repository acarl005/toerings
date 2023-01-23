//! This is the main file to house data collection functions.

use std::{
    net::IpAddr,
    time::{Duration, Instant},
};

use futures::join;

#[cfg(target_os = "linux")]
use fxhash::FxHashMap;

use serde::Serialize;
#[cfg(feature = "battery")]
use starship_battery::{Battery, Manager};

use sysinfo::{System, SystemExt};

#[cfg(feature = "nvidia")]
pub mod nvidia;

#[cfg(feature = "battery")]
pub mod batteries;
pub mod cpu;
pub mod disks;
pub mod memory;
pub mod network;
pub mod processes;
pub mod temperature;

#[derive(Clone, Debug, Serialize)]
pub struct Data {
    #[serde(with = "serde_millis")]
    pub last_collection_time: Instant,
    pub cpu: Option<cpu::CpuHarvest>,
    pub load_avg: Option<cpu::LoadAvgHarvest>,
    pub memory: Option<memory::MemHarvest>,
    pub swap: Option<memory::MemHarvest>,
    pub temperature_sensors: Option<Vec<temperature::TempHarvest>>,
    pub network: Option<network::NetworkHarvest>,
    pub list_of_processes: Option<Vec<processes::ProcessHarvest>>,
    pub disks: Option<Vec<disks::DiskHarvest>>,
    pub io: Option<disks::IoHarvest>,
    #[serde(with = "humantime_serde")]
    #[serde(default)]
    pub uptime: Duration,
    pub hostname: Option<String>,
    pub kernel_name: Option<String>,
    pub kernel_version: Option<String>,
    pub os_version: Option<String>,
    pub local_ip: Option<IpAddr>,
    #[cfg(feature = "battery")]
    pub list_of_batteries: Option<Vec<batteries::BatteryHarvest>>,
    #[cfg(feature = "zfs")]
    pub arc: Option<memory::MemHarvest>,
    #[cfg(feature = "gpu")]
    pub gpu: Option<Vec<(String, memory::MemHarvest)>>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            last_collection_time: Instant::now(),
            cpu: None,
            load_avg: None,
            memory: None,
            swap: None,
            temperature_sensors: None,
            list_of_processes: None,
            disks: None,
            io: None,
            network: None,
            uptime: Duration::ZERO,
            hostname: None,
            kernel_name: None,
            kernel_version: None,
            os_version: None,
            local_ip: None,
            #[cfg(feature = "battery")]
            list_of_batteries: None,
            #[cfg(feature = "zfs")]
            arc: None,
            #[cfg(feature = "gpu")]
            gpu: None,
        }
    }
}

impl Data {
    pub fn cleanup(&mut self) {
        self.io = None;
        self.temperature_sensors = None;
        self.list_of_processes = None;
        self.disks = None;
        self.memory = None;
        self.swap = None;
        self.cpu = None;
        self.load_avg = None;

        if let Some(network) = &mut self.network {
            network.first_run_cleanup();
        }
        #[cfg(feature = "zfs")]
        {
            self.arc = None;
        }
        #[cfg(feature = "gpu")]
        {
            self.gpu = None;
        }
    }
}

#[derive(Debug)]
pub struct DataCollector {
    pub data: Data,
    sys: System,
    previous_cpu_times: Vec<(cpu::PastCpuWork, cpu::PastCpuTotal)>,
    previous_average_cpu_time: Option<(cpu::PastCpuWork, cpu::PastCpuTotal)>,
    #[cfg(target_os = "linux")]
    pid_mapping: FxHashMap<crate::Pid, processes::PrevProcDetails>,
    #[cfg(target_os = "linux")]
    prev_idle: f64,
    #[cfg(target_os = "linux")]
    prev_non_idle: f64,
    mem_total_kb: u64,
    use_current_cpu_total: bool,
    unnormalized_cpu: bool,
    last_collection_time: Instant,
    total_rx: u64,
    total_tx: u64,
    show_average_cpu: bool,
    #[cfg(feature = "battery")]
    battery_manager: Option<Manager>,
    #[cfg(feature = "battery")]
    battery_list: Option<Vec<Battery>>,
    #[cfg(target_family = "unix")]
    user_table: self::processes::UserTable,
}

impl DataCollector {
    pub fn new() -> Self {
        DataCollector {
            data: Data::default(),
            sys: System::new_with_specifics(sysinfo::RefreshKind::new()),
            previous_cpu_times: vec![],
            previous_average_cpu_time: None,
            #[cfg(target_os = "linux")]
            pid_mapping: FxHashMap::default(),
            #[cfg(target_os = "linux")]
            prev_idle: 0_f64,
            #[cfg(target_os = "linux")]
            prev_non_idle: 0_f64,
            mem_total_kb: 0,
            use_current_cpu_total: false,
            unnormalized_cpu: false,
            last_collection_time: Instant::now(),
            total_rx: 0,
            total_tx: 0,
            show_average_cpu: false,
            #[cfg(feature = "battery")]
            battery_manager: None,
            #[cfg(feature = "battery")]
            battery_list: None,
            #[cfg(target_family = "unix")]
            user_table: Default::default(),
        }
    }

    pub fn init(&mut self) {
        #[cfg(target_os = "linux")]
        {
            futures::executor::block_on(self.initialize_memory_size());
        }
        #[cfg(not(target_os = "linux"))]
        {
            self.sys.refresh_memory();
            self.mem_total_kb = self.sys.total_memory();

            // TODO: Would be good to get this and network list running on a timer instead...?
            // Refresh components list once...
            self.sys.refresh_components_list();

            // Refresh network list once...
            if cfg!(target_os = "windows") {
                self.sys.refresh_networks_list();
            }

            self.sys.refresh_cpu();

            // Refresh disk list once...
            if cfg!(target_os = "freebsd") {
                self.sys.refresh_disks_list();
            }
        }

        #[cfg(feature = "battery")]
        {
            if let Ok(battery_manager) = Manager::new() {
                if let Ok(batteries) = battery_manager.batteries() {
                    let battery_list: Vec<Battery> = batteries.filter_map(Result::ok).collect();
                    if !battery_list.is_empty() {
                        self.battery_list = Some(battery_list);
                        self.battery_manager = Some(battery_manager);
                    }
                }
            }
        }

        futures::executor::block_on(self.update_data());

        std::thread::sleep(std::time::Duration::from_millis(250));

        self.data.cleanup();
    }

    #[cfg(target_os = "linux")]
    async fn initialize_memory_size(&mut self) {
        self.mem_total_kb = if let Ok(mem) = heim::memory::memory().await {
            mem.total().get::<heim::units::information::kilobyte>()
        } else {
            1
        };
    }

    pub async fn update_data(&mut self) {
        #[cfg(not(target_os = "linux"))]
        {
            self.sys.refresh_cpu();
            self.sys.refresh_processes();
            self.sys.refresh_components();

            #[cfg(target_os = "windows")]
            {
                self.sys.refresh_networks();
            }

            #[cfg(target_os = "freebsd")]
            {
                self.sys.refresh_disks();
                self.sys.refresh_memory();
            }
        }

        let current_instant = std::time::Instant::now();

        // CPU
        #[cfg(not(target_os = "freebsd"))]
        {
            if let Ok(cpu_data) = cpu::get_cpu_data_list(
                self.show_average_cpu,
                &mut self.previous_cpu_times,
                &mut self.previous_average_cpu_time,
            )
            .await
            {
                self.data.cpu = Some(cpu_data);
            }
        }
        #[cfg(target_os = "freebsd")]
        {
            if let Ok(cpu_data) = cpu::get_cpu_data_list(
                &self.sys,
                self.show_average_cpu,
                &mut self.previous_cpu_times,
                &mut self.previous_average_cpu_time,
            )
            .await
            {
                self.data.cpu = Some(cpu_data);
            }
        }

        #[cfg(target_family = "unix")]
        {
            // Load Average
            if let Ok(load_avg_data) = cpu::get_load_avg().await {
                self.data.load_avg = Some(load_avg_data);
            }
        }

        // Batteries
        #[cfg(feature = "battery")]
        {
            if let Some(battery_manager) = &self.battery_manager {
                if let Some(battery_list) = &mut self.battery_list {
                    self.data.list_of_batteries =
                        Some(batteries::refresh_batteries(battery_manager, battery_list));
                }
            }
        }

        if let Ok(mut process_list) = {
            #[cfg(target_os = "linux")]
            {
                // Must do this here since we otherwise have to make `get_process_data` async.
                use self::processes::CpuUsageStrategy;

                let normalize_cpu = if self.unnormalized_cpu {
                    heim::cpu::logical_count()
                        .await
                        .map(|v| CpuUsageStrategy::NonNormalized(v as f64))
                        .unwrap_or(CpuUsageStrategy::Normalized)
                } else {
                    CpuUsageStrategy::Normalized
                };

                processes::get_process_data(
                    &mut self.prev_idle,
                    &mut self.prev_non_idle,
                    &mut self.pid_mapping,
                    self.use_current_cpu_total,
                    normalize_cpu,
                    current_instant
                        .duration_since(self.last_collection_time)
                        .as_secs(),
                    self.mem_total_kb,
                    &mut self.user_table,
                )
            }
            #[cfg(not(target_os = "linux"))]
            {
                #[cfg(target_family = "unix")]
                {
                    processes::get_process_data(
                        &self.sys,
                        self.use_current_cpu_total,
                        self.unnormalized_cpu,
                        self.mem_total_kb,
                        &mut self.user_table,
                    )
                }
                #[cfg(not(target_family = "unix"))]
                {
                    processes::get_process_data(
                        &self.sys,
                        self.use_current_cpu_total,
                        self.unnormalized_cpu,
                        self.mem_total_kb,
                    )
                }
            }
        } {
            // NB: To avoid duplicate sorts on rerenders/events, we sort the processes by PID here.
            // We also want to avoid re-sorting *again* later on if we're sorting by PID, since we already
            // did it here!
            process_list.sort_unstable_by_key(|p| p.pid);
            self.data.list_of_processes = Some(process_list);
        }

        #[cfg(not(target_os = "linux"))]
        {
            if let Ok(data) = temperature::get_temperature_data(&self.sys) {
                self.data.temperature_sensors = data;
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(data) = temperature::get_temperature_data() {
                self.data.temperature_sensors = data;
            }
        }

        let network_data_fut = {
            #[cfg(any(target_os = "windows", target_os = "freebsd"))]
            {
                network::get_network_data(
                    &self.sys,
                    self.last_collection_time,
                    &mut self.total_rx,
                    &mut self.total_tx,
                    current_instant,
                )
            }
            #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
            {
                network::get_network_data(
                    self.last_collection_time,
                    &mut self.total_rx,
                    &mut self.total_tx,
                    current_instant,
                )
            }
        };
        let mem_data_fut = {
            #[cfg(not(target_os = "freebsd"))]
            {
                memory::get_mem_data()
            }
            #[cfg(target_os = "freebsd")]
            {
                memory::get_mem_data(&self.sys)
            }
        };
        let disk_data_fut = disks::get_disk_usage();
        let disk_io_usage_fut = disks::get_io_usage();

        let (net_data, mem_res, disk_res, io_res) = join!(
            network_data_fut,
            mem_data_fut,
            disk_data_fut,
            disk_io_usage_fut,
        );

        if let Ok(net_data) = net_data {
            if let Some(net_data) = &net_data {
                self.total_rx = net_data.total_rx;
                self.total_tx = net_data.total_tx;
            }
            self.data.network = net_data;
        }

        if let Ok(memory) = mem_res.ram {
            self.data.memory = memory;
        }

        if let Ok(swap) = mem_res.swap {
            self.data.swap = swap;
        }

        #[cfg(feature = "zfs")]
        if let Ok(arc) = mem_res.arc {
            self.data.arc = arc;
        }

        #[cfg(feature = "gpu")]
        if let Ok(gpu) = mem_res.gpus {
            self.data.gpu = gpu;
        }

        if let Ok(disks) = disk_res {
            self.data.disks = disks;
        }

        if let Ok(io) = io_res {
            self.data.io = io;
        }

        self.data.uptime = Duration::from_secs(self.sys.uptime());
        self.data.hostname = self.sys.host_name();
        self.data.kernel_name = self.sys.name();
        self.data.kernel_version = self.sys.kernel_version();
        self.data.os_version = self.sys.long_os_version();
        self.data.local_ip = local_ip_address::local_ip().ok();

        // Update time
        self.data.last_collection_time = current_instant;
        self.last_collection_time = current_instant;
    }
}

#[cfg(target_os = "freebsd")]
/// Deserialize [libxo](https://www.freebsd.org/cgi/man.cgi?query=libxo&apropos=0&sektion=0&manpath=FreeBSD+13.1-RELEASE+and+Ports&arch=default&format=html) JSON data
fn deserialize_xo<T>(key: &str, data: &[u8]) -> Result<T, std::io::Error>
where
    T: serde::de::DeserializeOwned,
{
    let mut value: serde_json::Value = serde_json::from_slice(data)?;
    value
        .as_object_mut()
        .and_then(|map| map.remove(key))
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "key not found"))
        .and_then(|val| serde_json::from_value(val).map_err(|err| err.into()))
}
