use std::process::Command;

fn main() {
    println!("Clipboard History Tool\n");
    println!("Note: This is a simple demonstration. macOS doesn't provide direct");
    println!("clipboard history access. For full clipboard history, consider using");
    println!("tools like Flycut, CopyClip, or Maccy.\n");

    // Get current clipboard content
    let output = Command::new("pbpaste")
        .output()
        .expect("Failed to access clipboard");

    let clipboard_content = String::from_utf8_lossy(&output.stdout);

    if clipboard_content.trim().is_empty() {
        println!("Clipboard is empty");
    } else {
        println!("Current clipboard content:");
        println!("{}", "-".repeat(70));

        // Truncate if too long
        let truncated = if clipboard_content.len() > 500 {
            format!("{}...\n(truncated, {} total characters)",
                   &clipboard_content[..500],
                   clipboard_content.len())
        } else {
            clipboard_content.to_string()
        };

        println!("{}", truncated);
        println!();
    }
}
