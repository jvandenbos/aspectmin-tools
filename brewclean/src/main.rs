use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("Homebrew Cache Cleaner\n");

    // Check if brew is installed
    let brew_check = Command::new("which")
        .arg("brew")
        .output();

    if brew_check.is_err() || !brew_check.unwrap().status.success() {
        eprintln!("Error: Homebrew not found. Please install Homebrew first.");
        std::process::exit(1);
    }

    // Get cache info
    println!("Checking Homebrew cache...\n");

    let cache_output = Command::new("brew")
        .arg("--cache")
        .output()
        .expect("Failed to get brew cache location");

    let cache_dir = String::from_utf8_lossy(&cache_output.stdout).trim().to_string();

    println!("Cache location: {}\n", cache_dir);

    // Show cleanup preview
    let cleanup_output = Command::new("brew")
        .arg("cleanup")
        .arg("--prune=all")
        .arg("-n")
        .output()
        .expect("Failed to run brew cleanup preview");

    let preview = String::from_utf8_lossy(&cleanup_output.stdout);

    if preview.trim().is_empty() {
        println!("✓ Brew cache is already clean!");
        return;
    }

    println!("Files that will be removed:");
    println!("{}", preview);

    print!("\nProceed with cleanup? (y/N): ");
    io::stdout().flush().unwrap();

    let mut response = String::new();
    io::stdin().read_line(&mut response).unwrap();

    if response.trim().to_lowercase() != "y" {
        println!("Cancelled.");
        return;
    }

    println!("\nCleaning...");

    let cleanup_result = Command::new("brew")
        .arg("cleanup")
        .arg("--prune=all")
        .status()
        .expect("Failed to run brew cleanup");

    if cleanup_result.success() {
        println!("\n✓ Homebrew cache cleaned successfully!");
    } else {
        eprintln!("Error: Cleanup failed");
        std::process::exit(1);
    }
}
