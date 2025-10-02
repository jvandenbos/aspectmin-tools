# JSON Support Implementation Guide

## Completed (5/18)
- ✅ diskfree
- ✅ portfind
- ✅ procmem
- ✅ cpuhog
- ✅ localip
- ✅ portcheck

## Tools Needing JSON

### Simple Output Tools
These can skip JSON (single-purpose, non-data tools):
- dnsflush - Just flushes cache (operation result)
- cleanxcode - Interactive cleanup (operation result)
- brewclean - Interactive cleanup (operation result)
- timer - Interactive countdown
- clipboard - Simple text display
- qr - Visual output only

### Data Tools Needing JSON (7 remaining)

**bigdirs** - Array of directories
```json
[
  {"path": "/foo/bar", "size_bytes": 123456, "size_human": "120 KB"},
  ...
]
```

**oldfiles** - Array of old files
```json
[
  {"path": "/foo/file.txt", "last_accessed": "2024-01-01T12:00:00", "days_old": 30},
  ...
]
```

**dupes** - Groups of duplicates
```json
[
  {
    "hash": "abc123...",
    "size_bytes": 1024,
    "files": ["/path/1", "/path/2"]
  },
  ...
]
```

**envdiff** - Diff results
```json
{
  "file1": ".env",
  "file2": ".env.example",
  "only_in_file1": ["KEY1", "KEY2"],
  "only_in_file2": ["KEY3"],
  "common": ["SHARED_KEY"]
}
```

**certinfo** - Certificate details
```json
{
  "subject": "example.com",
  "issuer": "Let's Encrypt",
  "valid_from": "2024-01-01",
  "valid_to": "2025-01-01",
  "days_remaining": 180
}
```

**appspace** - App sizes
```json
[
  {"app": "Xcode.app", "size_bytes": 123456789, "size_human": "117 MB"},
  ...
]
```

## Implementation Priority

HIGH (for LLMs):
1. bigdirs
2. dupes
3. envdiff
4. appspace

MEDIUM:
5. oldfiles
6. certinfo

LOW (operations):
7-12. dnsflush, cleanxcode, brewclean, timer, clipboard, qr
