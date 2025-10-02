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
    // Parse the certificate data from s_client output
    // Look for the certificate section
    if let Some(start) = data.find("-----BEGIN CERTIFICATE-----") {
        if let Some(end) = data[start..].find("-----END CERTIFICATE-----") {
            let cert_pem = &data[start..start + end + "-----END CERTIFICATE-----".len()];

            // Use openssl to display cert info
            let output = Command::new("openssl")
                .arg("x509")
                .arg("-noout")
                .arg("-subject")
                .arg("-issuer")
                .arg("-dates")
                .arg("-fingerprint")
                .stdin(std::process::Stdio::piped())
                .output();

            match output {
                Ok(mut child_output) => {
                    // Write cert to stdin
                    if let Ok(mut process) = Command::new("openssl")
                        .arg("x509")
                        .arg("-noout")
                        .arg("-subject")
                        .arg("-issuer")
                        .arg("-dates")
                        .arg("-fingerprint")
                        .stdin(std::process::Stdio::piped())
                        .stdout(std::process::Stdio::piped())
                        .spawn()
                    {
                        if let Some(mut stdin) = process.stdin.take() {
                            use std::io::Write;
                            let _ = stdin.write_all(cert_pem.as_bytes());
                        }

                        if let Ok(output) = process.wait_with_output() {
                            println!("{}", String::from_utf8_lossy(&output.stdout));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing certificate: {}", e);
                }
            }
        }
    }
}
