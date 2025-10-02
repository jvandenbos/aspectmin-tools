use std::env;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = env::args().collect();

    let json_mode = args.iter().any(|arg| arg == "--json" || arg == "-j");
    let args_without_flags: Vec<&String> = args.iter()
        .filter(|arg| *arg != "--json" && *arg != "-j" && !arg.starts_with('-'))
        .collect();

    if args_without_flags.len() != 3 {
        eprintln!("Usage: portcheck [--json] <host> <port>");
        eprintln!("Example: portcheck google.com 443");
        eprintln!("         portcheck --json google.com 443");
        std::process::exit(1);
    }

    let host = args_without_flags[1];
    let port = args_without_flags[2];

    let address = format!("{}:{}", host, port);

    // Resolve the address
    let socket_addrs = match address.to_socket_addrs() {
        Ok(addrs) => addrs,
        Err(e) => {
            eprintln!("Failed to resolve {}: {}", host, e);
            std::process::exit(1);
        }
    };

    for socket_addr in socket_addrs {
        let start = Instant::now();

        match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(5)) {
            Ok(_) => {
                let elapsed = start.elapsed();
                if json_mode {
                    println!("{{");
                    println!("  \"host\": \"{}\",", host);
                    println!("  \"port\": {},", port);
                    println!("  \"status\": \"open\",");
                    println!("  \"response_time_ms\": {}", elapsed.as_millis());
                    println!("}}");
                } else {
                    println!("✓ Port {} on {} is OPEN ({}ms)",
                             port, host, elapsed.as_millis());
                }
                return;
            }
            Err(e) => {
                let elapsed = start.elapsed();
                if json_mode {
                    println!("{{");
                    println!("  \"host\": \"{}\",", host);
                    println!("  \"port\": {},", port);
                    println!("  \"status\": \"closed\",");
                    println!("  \"response_time_ms\": {},", elapsed.as_millis());
                    println!("  \"error\": \"{}\"", e.to_string().replace("\"", "\\\""));
                    println!("}}");
                } else {
                    println!("✗ Port {} on {} is CLOSED ({}ms)",
                             port, host, elapsed.as_millis());
                    println!("  Error: {}", e);
                }
                return;
            }
        }
    }
}
