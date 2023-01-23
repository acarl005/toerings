/* eslint no-unused-vars: 0 */
/// <reference types="svelte" />
/// <reference types="vite/client" />

interface CPUData {
  cpu_usage: number
}

interface Process {
  name: string
  command: string
  pid: string
  parent_pid: string | null
  cpu_usage_percent: number
  mem_usage_percent: number
  mem_usage_bytes: number
  read_bytes_per_sec: number
  write_bytes_per_sec: number
  total_read_bytes: number
  total_write_bytes: number
  process_state: [string, string]
  uid: string | null
  user: string | null
}

interface DiskData {
  name: string
  mount_point: string
  free_space: number | null
  used_space: number | null
  total_space: number | null
}

interface MemData {
  mem_total_in_kib: number
  mem_used_in_kib: number
  use_percent: number | null
}

interface NetData {
  rx: number
  tx: number
  total_rx: number
  total_tx: number
}

interface TempData {
  name: string
  temperature: number
}

interface BatteryData {
  charge_percent: number
  secs_until_full: number | null
  secs_until_empty: number | null
  power_consumption_rate_watts: number
  health_percent: number
}

interface SummaryData {
  uptime: string
  hostname: string | null
  kernel_name: string | null
  kernel_version: string | null
  os_version: string | null
}

interface IOData {
  read_bytes: number
  write_bytes: number
}

interface Data {
  last_collection_time: number
  uptime: string
  hostname: string | null
  kernel_name: string | null
  kernel_version: string | null
  os_version: string | null
  list_of_processes: Array<Process>
  cpu: Array<CPUData>
  load_avg: Array<number>
  memory: MemData
  swap: MemData
  disks: Array<DiskData>
  io: Record<string, IOData>
  local_ip: string | null
  network: NetData
  temperature_sensors: Array<TempData>
  list_of_batteries?: Array<BatteryData>
  arc?: MemData
  gpu?: Array<[string, MemData]>
}
