use qrcode::QrCode;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: qr <text|url>");
        eprintln!("Example: qr \"https://example.com\"");
        std::process::exit(1);
    }

    let data = &args[1];

    let code = match QrCode::new(data) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error generating QR code: {}", e);
            std::process::exit(1);
        }
    };

    println!();
    let string = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();

    println!("{}", string);
    println!();
}
