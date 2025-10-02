use std::env;
use std::fs;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let (input, write_to_file) = if args.len() == 1 {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        (buffer, None)
    } else if args.len() == 2 {
        // Read from file
        let filename = &args[1];
        let content = fs::read_to_string(filename)
            .expect(&format!("Failed to read file: {}", filename));
        (content, None)
    } else if args.len() == 3 && args[1] == "--write" {
        // Read from file and write back
        let filename = &args[2];
        let content = fs::read_to_string(filename)
            .expect(&format!("Failed to read file: {}", filename));
        (content, Some(filename.clone()))
    } else {
        eprintln!("Usage:");
        eprintln!("  jsonfix <file>           - Validate and pretty-print JSON file");
        eprintln!("  jsonfix --write <file>   - Validate and write pretty JSON back to file");
        eprintln!("  cat file.json | jsonfix  - Read from stdin");
        std::process::exit(1);
    };

    match serde_json::from_str::<serde_json::Value>(&input) {
        Ok(json) => {
            let pretty = serde_json::to_string_pretty(&json)
                .expect("Failed to serialize JSON");

            if let Some(filename) = write_to_file {
                fs::write(&filename, &pretty)
                    .expect(&format!("Failed to write to {}", filename));
                println!("✓ JSON validated and written to {}", filename);
            } else {
                println!("{}", pretty);
            }
        }
        Err(e) => {
            eprintln!("✗ Invalid JSON:");
            eprintln!("  Error: {}", e);
            std::process::exit(1);
        }
    }
}
