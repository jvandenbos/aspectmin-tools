#!/bin/bash

echo "Installing Aspectmin tools..."
echo

INSTALL_DIR="$HOME/bin/aspectmin"
mkdir -p "$INSTALL_DIR"

TOOLS=(
  portfind procmem cpuhog
  localip dnsflush portcheck
  bigdirs oldfiles dupes
  envdiff jsonfix certinfo
  appspace cleanxcode brewclean
  timer clipboard qr
)

for tool in "${TOOLS[@]}"; do
  if [ -f "$tool/target/release/$tool" ]; then
    cp "$tool/target/release/$tool" "$INSTALL_DIR/"
    echo "✓ Installed $tool"
  else
    echo "⚠ $tool not built yet (run build-all.sh first)"
  fi
done

echo
echo "✅ Installation complete!"
echo "Tools installed to: $INSTALL_DIR"
echo
echo "Add to your PATH by adding this to ~/.zshrc or ~/.bashrc:"
echo "  export PATH=\"\$HOME/bin/aspectmin:\$PATH\""
