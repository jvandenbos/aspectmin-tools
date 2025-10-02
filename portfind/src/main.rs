use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (port, json_mode) = if args.len() == 2 {
        (&args[1], false)
    } else if args.len() == 3 && (args[1] == "--json" || args[1] == "-j") {
        (&args[2], true)
    } else if args.len() == 3 && (args[2] == "--json" || args[2] == "-j") {
        (&args[1], true)
    } else {
        eprintln!("Usage: portfind [--json] <port>");
        eprintln!("Example: portfind 3000");
        eprintln!("         portfind --json 3000");
        std::process::exit(1);
    };

    let port = port;

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
                    if json_mode {
                        println!("[]");
                    } else {
                        println!("No process found using port {}", port);
                    }
                    return;
                }

                if json_mode {
                    print!("[");
                    let mut first = true;
                    for line in lines.iter().skip(1) {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 9 {
                            if !first {
                                print!(",");
                            }
                            first = false;
                            println!();
                            println!("  {{");
                            println!("    \"process\": \"{}\",", parts[0]);
                            println!("    \"pid\": {},", parts[1]);
                            println!("    \"user\": \"{}\",", parts[2]);
                            println!("    \"type\": \"{}\"", parts[7]);
                            print!("  }}");
                        }
                    }
                    if !first {
                        println!();
                    }
                    println!("]");
                } else {
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
                }
            } else {
                if json_mode {
                    println!("[]");
                } else {
                    println!("No process found using port {}", port);
                }
            }
        }
        Err(e) => {
            eprintln!("Error running lsof: {}", e);
            eprintln!("Make sure lsof is installed on your system.");
            std::process::exit(1);
        }
    }
}
