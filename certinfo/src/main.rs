use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: certinfo <host|file>");
        eprintln!("Examples:");
        eprintln!("  certinfo example.com");
        eprintln!("  certinfo cert.pem");
        std::process::exit(1);
    }

    let input = &args[1];

    // Check if it's a file or hostname
    if std::path::Path::new(input).exists() {
        show_cert_from_file(input);
    } else {
        show_cert_from_host(input);
    }
}

fn show_cert_from_host(host: &str) {
    println!("Fetching certificate for {}...\n", host);

    let output = Command::new("openssl")
        .arg("s_client")
        .arg("-connect")
        .arg(format!("{}:443", host))
        .arg("-servername")
        .arg(host)
        .arg("-showcerts")
        .stdin(std::process::Stdio::null())
        .output();

    match output {
        Ok(output) => {
            let cert_data = String::from_utf8_lossy(&output.stdout);
            parse_and_display_cert(&cert_data);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Make sure openssl is installed");
            std::process::exit(1);
        }
    }
}

fn show_cert_from_file(path: &str) {
    println!("Reading certificate from {}...\n", path);

    let output = Command::new("openssl")
        .arg("x509")
        .arg("-in")
        .arg(path)
        .arg("-text")
        .arg("-noout")
        .output();

    match output {
        Ok(output) => {
            let cert_text = String::from_utf8_lossy(&output.stdout);
            println!("{}", cert_text);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Make sure openssl is installed");
            std::process::exit(1);
        }
    }
}

fn parse_and_display_cert(data: &str) {
    // Extract cert and pipe to openssl x509
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo \"\" | openssl s_client -connect $1:443 -servername $1 2>/dev/null | openssl x509 -noout -subject -issuer -dates")
        .arg("sh")
        .arg(data)
        .output();

    match output {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Err(e) => {
            eprintln!("Error parsing certificate: {}", e);
        }
    }
}
