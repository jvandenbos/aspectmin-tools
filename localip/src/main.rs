use pnet_datalink;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let json_mode = args.iter().any(|arg| arg == "--json" || arg == "-j");

    let interfaces = pnet_datalink::interfaces();
    let mut results = Vec::new();

    for interface in interfaces {
        // Skip loopback
        if interface.is_loopback() {
            continue;
        }

        for ip in interface.ips {
            let ip_addr = ip.ip();

            // Skip link-local addresses
            if ip_addr.is_loopback() {
                continue;
            }

            // Skip IPv6 link-local (fe80::)
            if let std::net::IpAddr::V6(addr) = ip_addr {
                if addr.segments()[0] == 0xfe80 {
                    continue;
                }
            }

            results.push((interface.name.clone(), ip_addr.to_string()));
        }
    }

    if json_mode {
        print!("[");
        for (i, (iface, ip)) in results.iter().enumerate() {
            if i > 0 {
                print!(",");
            }
            println!();
            println!("  {{");
            println!("    \"interface\": \"{}\",", iface);
            println!("    \"ip\": \"{}\"", ip);
            print!("  }}");
        }
        if !results.is_empty() {
            println!();
        }
        println!("]");
    } else {
        println!("{:<20} {}", "INTERFACE", "IP ADDRESS");
        println!("{}", "-".repeat(50));

        for (iface, ip) in results {
            println!("{:<20} {}", iface, ip);
        }
        println!();
    }
}
