use std::env;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: timer <duration>");
        eprintln!("Examples:");
        eprintln!("  timer 5m   (5 minutes)");
        eprintln!("  timer 30s  (30 seconds)");
        eprintln!("  timer 1h   (1 hour)");
        std::process::exit(1);
    }

    let input = &args[1];

    let seconds = match parse_duration(input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    println!("Timer started for {} ({} seconds)", input, seconds);
    println!("Press Ctrl+C to cancel\n");

    let start = std::time::Instant::now();
    let duration = Duration::from_secs(seconds);

    while start.elapsed() < duration {
        let remaining = duration - start.elapsed();
        let remaining_secs = remaining.as_secs();

        let hours = remaining_secs / 3600;
        let minutes = (remaining_secs % 3600) / 60;
        let secs = remaining_secs % 60;

        print!("\r⏱  {:02}:{:02}:{:02} remaining", hours, minutes, secs);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        thread::sleep(Duration::from_millis(100));
    }

    println!("\r✓ Timer finished!              ");

    // Send macOS notification
    let _ = Command::new("osascript")
        .arg("-e")
        .arg(format!("display notification \"Timer for {} has finished!\" with title \"Timer Complete\"", input))
        .output();
}

fn parse_duration(input: &str) -> Result<u64, String> {
    let input = input.trim().to_lowercase();

    if input.is_empty() {
        return Err("Duration cannot be empty".to_string());
    }

    let (num_str, unit) = if input.ends_with('s') {
        (&input[..input.len()-1], "s")
    } else if input.ends_with('m') {
        (&input[..input.len()-1], "m")
    } else if input.ends_with('h') {
        (&input[..input.len()-1], "h")
    } else {
        return Err("Duration must end with s (seconds), m (minutes), or h (hours)".to_string());
    };

    let num: u64 = num_str.parse()
        .map_err(|_| format!("Invalid number: {}", num_str))?;

    let seconds = match unit {
        "s" => num,
        "m" => num * 60,
        "h" => num * 3600,
        _ => return Err("Invalid unit".to_string()),
    };

    Ok(seconds)
}
