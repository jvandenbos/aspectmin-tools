use std::collections::HashMap;
use std::env;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    let json_mode = args.iter().any(|arg| arg == "--json" || arg == "-j");

    let current_dir = env::current_dir().expect("Failed to get current directory");

    if !json_mode {
        println!("Scanning directories in {}...\n", current_dir.display());
    }

    let mut dir_sizes: HashMap<String, u64> = HashMap::new();

    // Walk the directory tree
    for entry in WalkDir::new(&current_dir)
        .min_depth(1)
        .max_depth(3)
        .into_iter()
        .filter_entry(|e| {
            // Skip hidden directories
            !e.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
        })
    {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    // Get the parent directory
                    if let Some(parent) = entry.path().parent() {
                        let parent_str = parent.to_string_lossy().to_string();
                        *dir_sizes.entry(parent_str).or_insert(0) += metadata.len();
                    }
                }
            }
        }
    }

    // Convert to vector and sort by size
    let mut sorted_dirs: Vec<_> = dir_sizes.into_iter().collect();
    sorted_dirs.sort_by(|a, b| b.1.cmp(&a.1));

    if json_mode {
        print!("[");
        for (i, (dir, size)) in sorted_dirs.iter().take(10).enumerate() {
            if i > 0 {
                print!(",");
            }
            println!();
            println!("  {{");
            println!("    \"path\": \"{}\",", dir.replace("\"", "\\\""));
            println!("    \"size_bytes\": {},", size);
            println!("    \"size_human\": \"{}\"", format_bytes(*size));
            print!("  }}");
        }
        if !sorted_dirs.is_empty() {
            println!();
        }
        println!("]");
    } else {
        println!("{:>12} {}", "SIZE", "DIRECTORY");
        println!("{}", "-".repeat(70));

        for (dir, size) in sorted_dirs.iter().take(10) {
            println!("{:>12} {}", format_bytes(*size), dir);
        }
        println!();
    }
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
