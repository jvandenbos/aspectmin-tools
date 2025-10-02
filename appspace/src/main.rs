use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    let json_mode = args.iter().any(|arg| arg == "--json" || arg == "-j");

    let apps_dir = "/Applications";

    if !json_mode {
        println!("Scanning applications in {}...\n", apps_dir);
    }

    let mut app_sizes: Vec<(String, u64)> = Vec::new();

    if let Ok(entries) = fs::read_dir(apps_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("app") {
                let app_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown")
                    .to_string();

                let size = calculate_dir_size(&path);
                app_sizes.push((app_name, size));
            }
        }
    }

    // Sort by size descending
    app_sizes.sort_by(|a, b| b.1.cmp(&a.1));

    if json_mode {
        print!("[");
        for (i, (app_name, size)) in app_sizes.iter().take(20).enumerate() {
            if i > 0 {
                print!(",");
            }
            println!();
            println!("  {{");
            println!("    \"app\": \"{}\",", app_name.replace("\"", "\\\""));
            println!("    \"size_bytes\": {},", size);
            println!("    \"size_human\": \"{}\"", format_bytes(*size));
            print!("  }}");
        }
        if !app_sizes.is_empty() {
            println!();
        }
        println!("]");
    } else {
        println!("{:>12} {}", "SIZE", "APPLICATION");
        println!("{}", "-".repeat(70));

        for (app_name, size) in app_sizes.iter().take(20) {
            println!("{:>12} {}", format_bytes(*size), app_name);
        }
        println!();
    }
}

fn calculate_dir_size(path: &Path) -> u64 {
    let mut total_size = 0u64;

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                total_size += metadata.len();
            }
        }
    }

    total_size
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
