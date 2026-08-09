//lib.rs
// gpu discovery and stats

use nvml_wrapper::Nvml;
use nvml_wrapper::enums::device::UsedGpuMemory;
use std::fs;
use std::os::unix::fs::MetadataExt;

pub struct GpuProcess {
    pub pid: u32,
    pub used_memory_mb: Option<u64>,
    pub username: Option<String>,
}

pub struct GpuInfo {
    pub index: u32,
    pub name: String,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub utilization_pct: u32,
    pub processes: Vec<GpuProcess>,
}

pub fn get_username_for_pid(pid: u32) -> Option<String> {
    let path = format!("/proc/{}", pid);
    let metadata = fs::metadata(path).ok()?;
    let uid = metadata.uid();
    users::get_user_by_uid(uid).map(|u| u.name().to_string_lossy().into_owned())
}

pub fn list_gpus() -> Result<Vec<GpuInfo>, Box<dyn std::error::Error>> {
    let nvml = Nvml::init()?;
    let count = nvml.device_count()?;
    let mut gpus = Vec::new();

    for i in 0..count {
        let device = nvml.device_by_index(i)?;
        let mem = device.memory_info()?;
        let util = device.utilization_rates()?;

        let processes = device
            .running_compute_processes()?
            .into_iter()
            .map(|p| GpuProcess {
                pid: p.pid,
                used_memory_mb: match p.used_gpu_memory {
                    UsedGpuMemory::Used(bytes) => Some(bytes / 1024 / 1024),
                    UsedGpuMemory::Unavailable => None,
                },
                username: get_username_for_pid(p.pid),
            })
            .collect();

        gpus.push(GpuInfo {
            index: i,
            name: device.name()?,
            memory_used_mb: mem.used / 1024 / 1024,
            memory_total_mb: mem.total / 1024 / 1024,
            utilization_pct: util.gpu,
            processes,
        });
    }

    Ok(gpus)
}
