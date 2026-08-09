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
                        match p.used_memory_mb {
                            Some(mb) => println!("PID {} is using {} MB", p.pid, mb),
                            None => println!("PID {} USED IT ALLL", p.pid),
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("Error initialiing NVML: {e}"),
    }
}
