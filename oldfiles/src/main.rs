use chrono::{DateTime, Local};
use std::env;
use std::time::{SystemTime, Duration};
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: oldfiles <days>");
        eprintln!("Example: oldfiles 30  (files not accessed in 30+ days)");
        std::process::exit(1);
    }

    let days: u64 = match args[1].parse() {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Error: Invalid number of days");
            std::process::exit(1);
        }
    };

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let threshold = SystemTime::now() - Duration::from_secs(days * 24 * 60 * 60);

    println!("Finding files not accessed in {} days in {}...\n", days, current_dir.display());
    println!("{:<25} {}", "LAST ACCESSED", "FILE");
    println!("{}", "-".repeat(80));

    let mut found_count = 0;

    for entry in WalkDir::new(&current_dir)
        .into_iter()
        .filter_entry(|e| {
            // Skip hidden files/directories
            !e.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
        })
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                if let Ok(accessed) = metadata.accessed() {
                    if accessed < threshold {
                        let datetime: DateTime<Local> = accessed.into();
                        println!("{:<25} {}",
                                 datetime.format("%Y-%m-%d %H:%M:%S"),
                                 entry.path().display());
                        found_count += 1;

                        if found_count >= 50 {
                            println!("\n... (showing first 50 results)");
                            break;
                        }
                    }
                }
            }
        }
    }

    if found_count == 0 {
        println!("No files found older than {} days", days);
    }
    println!();
}
