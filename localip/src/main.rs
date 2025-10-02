use pnet_datalink;

fn main() {
    println!("{:<20} {}", "INTERFACE", "IP ADDRESS");
    println!("{}", "-".repeat(50));

    let interfaces = pnet_datalink::interfaces();

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

            println!("{:<20} {}", interface.name, ip_addr);
        }
    }
    println!();
}
