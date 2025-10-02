# Aspectmin Tools

A collection of 18 fast, simple CLI tools for macOS that solve common frustrations with built-in utilities.

Built in Rust for speed and reliability. Each tool does one thing well.

## 🚀 Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/aspectmin-tools
cd aspectmin-tools

# Build all tools (requires Rust)
./build-all.sh

# Or install pre-built binaries to ~/bin/aspectmin
./install.sh

# Add to your PATH (add to ~/.zshrc or ~/.bashrc)
export PATH="$HOME/bin/aspectmin:$PATH"
```

## 📦 Tools

### Process & Resource Management
- **portfind** - Find what's using a port
  ```bash
  portfind 3000
  ```

- **procmem** - Top 10 memory consumers
  ```bash
  procmem
  ```

- **cpuhog** - Top 10 CPU hogs
  ```bash
  cpuhog
  ```

### Network
- **localip** - Show all local IP addresses (WiFi, Ethernet, VPN)
  ```bash
  localip
  ```

- **dnsflush** - Flush DNS cache
  ```bash
  dnsflush
  ```

- **portcheck** - Quick port connectivity test
  ```bash
  portcheck google.com 443
  ```

### File System
- **bigdirs** - Top 10 largest directories from current path
  ```bash
  bigdirs
  ```

- **oldfiles** - Find files not accessed in X days
  ```bash
  oldfiles 30
  ```

- **dupes** - Find duplicate files by SHA-256 hash
  ```bash
  dupes
  ```

### Development
- **envdiff** - Compare .env files safely (doesn't show values)
  ```bash
  envdiff .env .env.example
  ```

- **jsonfix** - Validate and pretty-print JSON
  ```bash
  jsonfix file.json
  jsonfix --write file.json
  cat data.json | jsonfix
  ```

- **certinfo** - Show SSL certificate details
  ```bash
  certinfo example.com
  certinfo cert.pem
  ```

### macOS-Specific
- **appspace** - Disk usage by macOS application
  ```bash
  appspace
  ```

- **cleanxcode** - Clean Xcode derived data and archives
  ```bash
  cleanxcode
  ```

- **brewclean** - Clean Homebrew caches
  ```bash
  brewclean
  ```

### Productivity
- **timer** - Simple countdown timer with notification
  ```bash
  timer 5m
  timer 30s
  timer 1h
  ```

- **clipboard** - View current clipboard content
  ```bash
  clipboard
  ```

- **qr** - Generate QR code in terminal
  ```bash
  qr "https://example.com"
  qr "some text"
  ```

## 🔧 Building from Source

Each tool is a separate Rust project:

```bash
cd portfind
cargo build --release
# Binary will be in target/release/portfind
```

Or build all at once:

```bash
for dir in */; do
  cd "$dir"
  cargo build --release
  cd ..
done
```

## 📝 Why These Tools?

macOS comes with great tools, but they can be frustrating:
- `lsof -i :3000` is hard to remember
- `df -h` shows too many system volumes
- DNS flushing requires a long command with sudo
- Finding large directories requires complex `du` incantations

These tools solve these frustrations with simple, memorable commands.

## 🤝 Contributing

Contributions welcome! Each tool is independent, so:
1. Pick a tool directory
2. Make your changes
3. Test with `cargo test` and `cargo build --release`
4. Submit a PR

## 📄 License

MIT License - feel free to use these tools however you'd like.

## 🙏 Credits

Built with ❤️ by Aspectmin

Generated with [Claude Code](https://claude.com/claude-code)
