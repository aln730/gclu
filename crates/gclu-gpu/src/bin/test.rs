fn main(){
    match gclu_gpu::list_gpus() {
        Ok(gpus) => {
            if gpus.is_empty(){
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
            }
        }
        Err(e) => eprintln!("Error initialiing NVML: {e}"),
    }
}
