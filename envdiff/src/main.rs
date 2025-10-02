use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: envdiff <file1> <file2>");
        eprintln!("Example: envdiff .env .env.example");
        std::process::exit(1);
    }

    let file1 = &args[1];
    let file2 = &args[2];

    let keys1 = read_env_keys(file1).expect(&format!("Failed to read {}", file1));
    let keys2 = read_env_keys(file2).expect(&format!("Failed to read {}", file2));

    let in1_not2: Vec<_> = keys1.difference(&keys2).collect();
    let in2_not1: Vec<_> = keys2.difference(&keys1).collect();

    println!("Comparing {} vs {}\n", file1, file2);

    if in1_not2.is_empty() && in2_not1.is_empty() {
        println!("✓ Files have the same keys!");
        return;
    }

    if !in1_not2.is_empty() {
        println!("Keys in {} but NOT in {}:", file1, file2);
        let mut sorted = in1_not2.clone();
        sorted.sort();
        for key in sorted {
            println!("  - {}", key);
        }
        println!();
    }

    if !in2_not1.is_empty() {
        println!("Keys in {} but NOT in {}:", file2, file1);
        let mut sorted = in2_not1.clone();
        sorted.sort();
        for key in sorted {
            println!("  + {}", key);
        }
        println!();
    }
}

fn read_env_keys(path: &str) -> io::Result<HashSet<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut keys = HashSet::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Extract key from KEY=VALUE
        if let Some(pos) = trimmed.find('=') {
            let key = trimmed[..pos].trim().to_string();
            keys.insert(key);
        }
    }

    Ok(keys)
}
