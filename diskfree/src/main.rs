use sysinfo::Disks;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let machine_readable = args.iter().any(|arg| arg == "--json" || arg == "-j");

    let disks = Disks::new_with_refreshed_list();

    if machine_readable {
        print_json(&disks);
    } else {
        print_human(&disks);
    }
}

fn print_human(disks: &Disks) {
    // Print header
    println!("{:<30} {:>10} {:>10} {:>10} {:>6}",
             "Mount Point", "Size", "Used", "Available", "Use%");
    println!("{}", "-".repeat(70));

    for disk in disks {
        let mount_point = disk.mount_point().to_string_lossy();

        // Filter out system/virtual filesystems on macOS
        if should_show_disk(&mount_point) {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total - available;
            let usage_percent = if total > 0 {
                (used as f64 / total as f64 * 100.0) as u32
            } else {
                0
            };

            println!("{:<30} {:>10} {:>10} {:>10} {:>5}%",
                     mount_point,
                     format_bytes(total),
                     format_bytes(used),
                     format_bytes(available),
                     usage_percent);
        }
    }
    println!();
}

fn print_json(disks: &Disks) {
    let mut first = true;
    print!("[");

    for disk in disks {
        let mount_point = disk.mount_point().to_string_lossy();

        if should_show_disk(&mount_point) {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total - available;
            let usage_percent = if total > 0 {
                (used as f64 / total as f64 * 100.0) as u32
            } else {
                0
            };

            if !first {
                print!(",");
            }
            first = false;

            println!();
            println!("  {{");
            println!("    \"mount_point\": \"{}\",", mount_point);
            println!("    \"total_bytes\": {},", total);
            println!("    \"used_bytes\": {},", used);
            println!("    \"available_bytes\": {},", available);
            println!("    \"usage_percent\": {}", usage_percent);
            print!("  }}");
        }
    }

    if !first {
        println!();
    }
    println!("]");
}

fn should_show_disk(mount_point: &str) -> bool {
    // Show root filesystem and anything in /Volumes
    // Skip system/virtual filesystems
    if mount_point.starts_with("/System/Volumes") {
        return false;
    }
    if mount_point.starts_with("/dev") {
        return false;
    }
    if mount_point.starts_with("/private") {
        return false;
    }
    if mount_point == "/" || mount_point.starts_with("/Volumes") {
        return true;
    }

    false
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

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
