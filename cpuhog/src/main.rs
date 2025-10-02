use sysinfo::System;
use std::{env, thread, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();
    let json_mode = args.iter().any(|arg| arg == "--json" || arg == "-j");

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

    if json_mode {
        print!("[");
        for (i, process) in processes.iter().take(10).enumerate() {
            if i > 0 {
                print!(",");
            }
            println!();
            println!("  {{");
            println!("    \"pid\": {},", process.pid());
            println!("    \"cpu_percent\": {:.1},", process.cpu_usage());
            println!("    \"process\": \"{}\"", process.name().to_string_lossy().replace("\"", "\\\""));
            print!("  }}");
        }
        if !processes.is_empty() {
            println!();
        }
        println!("]");
    } else {
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
}
