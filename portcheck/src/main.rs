use std::env;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: portcheck <host> <port>");
        eprintln!("Example: portcheck google.com 443");
        std::process::exit(1);
    }

    let host = &args[1];
    let port = &args[2];

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
                println!("✓ Port {} on {} is OPEN ({}ms)",
                         port, host, elapsed.as_millis());
                return;
            }
            Err(e) => {
                let elapsed = start.elapsed();
                println!("✗ Port {} on {} is CLOSED ({}ms)",
                         port, host, elapsed.as_millis());
                println!("  Error: {}", e);
                return;
            }
        }
    }
}
