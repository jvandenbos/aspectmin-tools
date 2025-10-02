use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: portfind <port>");
        eprintln!("Example: portfind 3000");
        std::process::exit(1);
    }

    let port = &args[1];

    // Use lsof to find what's using the port
    let output = Command::new("lsof")
        .arg("-i")
        .arg(format!(":{}", port))
        .arg("-P")
        .arg("-n")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = stdout.lines().collect();

                if lines.len() <= 1 {
                    println!("No process found using port {}", port);
                    return;
                }

                println!("{:<20} {:<10} {:<10} {}", "PROCESS", "PID", "USER", "TYPE");
                println!("{}", "-".repeat(60));

                for line in lines.iter().skip(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 9 {
                        let process = parts[0];
                        let pid = parts[1];
                        let user = parts[2];
                        let conn_type = parts[7];
                        println!("{:<20} {:<10} {:<10} {}", process, pid, user, conn_type);
                    }
                }
            } else {
                println!("No process found using port {}", port);
            }
        }
        Err(e) => {
            eprintln!("Error running lsof: {}", e);
            eprintln!("Make sure lsof is installed on your system.");
            std::process::exit(1);
        }
    }
}
