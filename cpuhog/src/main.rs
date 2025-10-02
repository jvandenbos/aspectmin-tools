use sysinfo::System;
use std::{thread, time::Duration};

fn main() {
    let mut sys = System::new_all();

    // Sleep to get accurate CPU readings
    thread::sleep(Duration::from_millis(500));
    sys.refresh_all();

    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| {
        let cpu_a = a.cpu_usage();
        let cpu_b = b.cpu_usage();
        cpu_b.partial_cmp(&cpu_a).unwrap_or(std::cmp::Ordering::Equal)
    });

    println!("{:<10} {:>8} {}", "PID", "CPU%", "PROCESS");
    println!("{}", "-".repeat(60));

    for process in processes.iter().take(10) {
        println!("{:<10} {:>7.1}% {}",
                 process.pid(),
                 process.cpu_usage(),
                 process.name().to_string_lossy());
    }
    println!();
}
