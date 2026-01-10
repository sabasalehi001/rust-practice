use sysinfo::{System, SystemExt, CpuExt, MemoryExt};

fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    println!("OS Name: {}", system.name().unwrap_or_else(|| "Unknown".to_string()));
    println!("OS Version: {}", system.os_version().unwrap_or_else(|| "Unknown".to_string()));
    println!("CPU Model: {}", system.cpus()[0].brand());
    println!("Total RAM: {} MB", system.total_memory() / 1024);
}