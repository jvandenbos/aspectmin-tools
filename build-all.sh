#!/bin/bash

echo "Building all Aspectmin tools..."
echo

TOOLS=(
  portfind procmem cpuhog
  localip dnsflush portcheck
  bigdirs oldfiles dupes
  envdiff jsonfix certinfo
  appspace cleanxcode brewclean
  timer clipboard qr
)

for tool in "${TOOLS[@]}"; do
  echo "Building $tool..."
  cd "$tool" && cargo build --release && cd ..
  if [ $? -ne 0 ]; then
    echo "❌ Failed to build $tool"
    exit 1
  fi
done

echo
echo "✅ All tools built successfully!"
echo "Binaries are in each tool's target/release/ directory"
