fn main() {
    match gclu_gpu::list_gpus() {
        Ok(gpus) => {
            if gpus.is_empty() {
                println!("No GPUs found. :(");
            }
            for gpu in gpus {
                println!(
                    "GPU {}: {}, {}/{} MB, {}%",
                    gpu.index,
                    gpu.name,
                    gpu.memory_used_mb,
                    gpu.memory_total_mb,
                    gpu.utilization_pct,
                );

                if gpu.processes.is_empty() {
                    println!("no processes!");
                } else {
                    for p in &gpu.processes {
                        let user = p.username.as_deref().unwrap_or("unknown");
                        let cmd = p.cmdline.as_deref().unwrap_or("unknown");
                        match p.used_memory_mb {
                            Some(mb) => println!("{} (PID {}) is using {} MB for {}", user, p.pid, mb, cmd),
                            None => println!("{} ({}) USED IT ALLL FOR {}", user, p.pid, cmd),
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("Error initialiing NVML: {e}"),
    }
}
