use sysinfo::{System, SystemExt, CpuExt}; // Only import SystemExt and CpuExt now
use chrono::Local;
use std::{thread, time::Duration};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "riscv_sysmon", about = "System monitor for RISC-V SBCs on Linux")]
struct Cli {
    #[structopt(short = "i", long = "interval", default_value = "1", help = "Update interval in seconds")]
    interval: u64,
}

fn main() {
    let args = Cli::from_args();
    let update_interval = Duration::from_secs(args.interval);

    println!("Starting riscv_sysmon...");
    let mut sys = System::new_all();

    loop {
        sys.refresh_all();
        print_system_info(&sys);
        thread::sleep(update_interval);
    }
}

/// Prints CPU and memory information from the system
fn print_system_info(sys: &System) {
    let local_time = Local::now();
    println!("\nSystem Metrics - {}", local_time.format("%Y-%m-%d %H:%M:%S"));
    println!("---------------------------");

    // CPU Information
    let cpu_usage: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;
    println!("CPU Usage: {:.2}%", cpu_usage);

    // Memory Information
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    println!("Memory Usage: {} / {} MB", used_memory / 1024, total_memory / 1024);
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysinfo::System;

    #[test]
    fn test_cpu_usage_calculation() {
        let mut sys = System::new_all();
        sys.refresh_cpu();

        let cpu_usage: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;
        assert!(cpu_usage >= 0.0 && cpu_usage <= 100.0, "CPU usage should be between 0 and 100%");
    }

    #[test]
    fn test_memory_usage_retrieval() {
        let mut sys = System::new_all();
        sys.refresh_memory();

        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        assert!(total_memory >= used_memory, "Total memory should be greater than or equal to used memory");
    }
}
