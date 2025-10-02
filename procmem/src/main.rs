use sysinfo::System;

fn main() {
    let sys = System::new_all();

    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| b.memory().cmp(&a.memory()));

    println!("{:<10} {:>12} {}", "PID", "MEMORY", "PROCESS");
    println!("{}", "-".repeat(60));

    for process in processes.iter().take(10) {
        let memory = format_bytes(process.memory());
        println!("{:<10} {:>12} {}",
                 process.pid(),
                 memory,
                 process.name().to_string_lossy());
    }
    println!();
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];

    if bytes == 0 {
        return "0 B".to_string();
    }

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if size >= 100.0 {
        format!("{:.0} {}", size, UNITS[unit_index])
    } else if size >= 10.0 {
        format!("{:.1} {}", size, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}
