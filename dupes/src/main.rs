use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use walkdir::WalkDir;

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");

    println!("Scanning for duplicate files in {}...\n", current_dir.display());

    let mut hashes: HashMap<String, Vec<String>> = HashMap::new();

    for entry in WalkDir::new(&current_dir)
        .into_iter()
        .filter_entry(|e| {
            !e.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
        })
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            if let Ok(hash) = hash_file(entry.path().to_str().unwrap()) {
                hashes
                    .entry(hash)
                    .or_insert_with(Vec::new)
                    .push(entry.path().display().to_string());
            }
        }
    }

    let mut found_dupes = false;

    for (hash, files) in hashes.iter() {
        if files.len() > 1 {
            found_dupes = true;

            if let Ok(metadata) = std::fs::metadata(&files[0]) {
                println!("Duplicate group ({}):", format_bytes(metadata.len()));
                println!("Hash: {}", &hash[..16]);
                for file in files {
                    println!("  - {}", file);
                }
                println!();
            }
        }
    }

    if !found_dupes {
        println!("No duplicate files found!");
    }
}

fn hash_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
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
