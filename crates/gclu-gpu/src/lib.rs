//lib.rs
// gpu discovery and stats

use nvml_wrapper::Nvml;

pub struct GpuInfo {
    pub index: u32,
    pub name: String,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub utilization_pct: u32,
}

pub fn list_gpus() -> Result<Vec<GpuInfo>, Box<dyn std::error::Error>> {
    let nvml = Nvml::init()?;
    let count = nvml.device_count()?;
    let mut gpus = Vec::new();

    for i in 0..count {
        let device = nvml.device_by_index(i)?;
        let mem = device.memory_info()?;
        let util = device.utilization_rates()?;

        gpus.push(GpuInfo {
            index: i,
            name: device.name()?,
            memory_used_mb: mem.used / 1024 / 1024,
            memory_total_mb: mem.total / 1024 / 1024,
            utilization_pct: util.gpu,
        });
    }

    Ok(gpus)
}
