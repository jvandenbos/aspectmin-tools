#!/bin/bash

# This script adds --json support implementation notes to each tool
# Manual implementation will be needed for each tool based on its output structure

cat << 'EOF'
JSON Support Implementation Plan:

Tools to update:
1. cpuhog - Similar to procmem (pid, cpu%, process)
2. localip - List of interfaces with IPs
3. portcheck - Connection status object
4. bigdirs - Array of directories with sizes
5. oldfiles - Array of files with timestamps
6. dupes - Groups of duplicate files
7. envdiff - Diff object with keys
8. certinfo - Certificate details object
9. appspace - Array of apps with sizes
10. cleanxcode - Operation result with sizes
11. brewclean - Operation result with files
12. timer - Timer status/completion
13. clipboard - Clipboard content object
14. qr - QR data (not applicable for visual output)

Each tool should accept --json or -j flag and output valid JSON.
EOF
