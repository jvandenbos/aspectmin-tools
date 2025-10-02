use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let home = env::var("HOME").expect("HOME environment variable not set");

    let derived_data_path = format!("{}/Library/Developer/Xcode/DerivedData", home);
    let archives_path = format!("{}/Library/Developer/Xcode/Archives", home);

    println!("Xcode Cache Cleaner\n");

    let derived_data_size = if Path::new(&derived_data_path).exists() {
        calculate_dir_size(&derived_data_path)
    } else {
        0
    };

    let archives_size = if Path::new(&archives_path).exists() {
        calculate_dir_size(&archives_path)
    } else {
        0
    };

    let total_size = derived_data_size + archives_size;

    if total_size == 0 {
        println!("No Xcode cache found!");
        return;
    }

    println!("Current Xcode cache size:");
    println!("  DerivedData: {}", format_bytes(derived_data_size));
    println!("  Archives:    {}", format_bytes(archives_size));
    println!("  Total:       {}\n", format_bytes(total_size));

    print!("Delete these files? (y/N): ");
    io::stdout().flush().unwrap();

    let mut response = String::new();
    io::stdin().read_line(&mut response).unwrap();

    if response.trim().to_lowercase() != "y" {
        println!("Cancelled.");
        return;
    }

    println!("\nCleaning...");

    let mut freed = 0u64;

    if Path::new(&derived_data_path).exists() {
        if let Err(e) = fs::remove_dir_all(&derived_data_path) {
            eprintln!("Error removing DerivedData: {}", e);
        } else {
            println!("✓ Removed DerivedData");
            freed += derived_data_size;
        }
    }

    if Path::new(&archives_path).exists() {
        if let Err(e) = fs::remove_dir_all(&archives_path) {
            eprintln!("Error removing Archives: {}", e);
        } else {
            println!("✓ Removed Archives");
            freed += archives_size;
        }
    }

    println!("\n✓ Freed {} of disk space!", format_bytes(freed));
}

fn calculate_dir_size(path: &str) -> u64 {
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
