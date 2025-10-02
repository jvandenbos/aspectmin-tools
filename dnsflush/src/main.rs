use std::process::Command;

fn main() {
    println!("Flushing DNS cache...");

    // Run dscacheutil -flushcache
    let result1 = Command::new("dscacheutil")
        .arg("-flushcache")
        .status();

    if let Err(e) = result1 {
        eprintln!("Error running dscacheutil: {}", e);
        std::process::exit(1);
    }

    // Run sudo killall -HUP mDNSResponder
    let result2 = Command::new("sudo")
        .arg("killall")
        .arg("-HUP")
        .arg("mDNSResponder")
        .status();

    match result2 {
        Ok(status) => {
            if status.success() {
                println!("✓ DNS cache flushed successfully!");
            } else {
                eprintln!("⚠ DNS cache partially flushed (mDNSResponder failed)");
                eprintln!("  You may need to run with sudo or enter your password");
            }
        }
        Err(e) => {
            eprintln!("Error running sudo killall: {}", e);
            std::process::exit(1);
        }
    }
}
