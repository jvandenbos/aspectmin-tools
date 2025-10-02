# Aspectmin Tools - Development Status

## ✅ Completed (Phase 1)

### Core Tools Built (18 total)
All tools are functional with release binaries in their respective `target/release/` directories.

**Process & Resource:**
- portfind ✅ (with `--json`)
- procmem ✅ (with `--json`)
- cpuhog ✅ (with `--json`)

**Network:**
- localip ✅ (with `--json`)
- dnsflush ✅
- portcheck ✅

**File System:**
- bigdirs ✅
- oldfiles ✅
- dupes ✅

**Development:**
- envdiff ✅
- jsonfix ✅ (has JSON handling built-in)
- certinfo ✅

**macOS-Specific:**
- appspace ✅
- cleanxcode ✅
- brewclean ✅

**Productivity:**
- timer ✅
- clipboard ✅
- qr ✅

**Bonus:**
- diskfree ✅ (with `--json`)

## 🚧 In Progress (Phase 2)

### JSON Support
- [x] portfind
- [x] procmem
- [x] cpuhog
- [x] localip
- [x] diskfree (already had it)
- [ ] dnsflush
- [ ] portcheck
- [ ] bigdirs
- [ ] oldfiles
- [ ] dupes
- [ ] envdiff
- [ ] certinfo
- [ ] appspace
- [ ] cleanxcode
- [ ] brewclean
- [ ] timer
- [ ] clipboard
- [ ] qr (N/A - visual output)

### Man Pages
- [ ] Generate man pages for all 18 tools
- [ ] Install man pages to ~/bin/aspectmin/man/

## 📋 Planned (Phase 3)

### LLM-Focused Tools Suite
New repository: `aspectmin-llm-tools`

**Code Understanding:**
- [ ] treemap - Project structure with metadata
- [ ] codebase - Codebase statistics
- [ ] funclist - List all functions/classes
- [ ] depgraph - Dependency graph
- [ ] apimap - Map all API endpoints

**Testing & Validation:**
- [ ] testfind - Locate test files
- [ ] testrun - Run tests by pattern
- [ ] coverage - Quick coverage report

**Git & Changes:**
- [ ] gitdiff - Enhanced diff with stats
- [ ] gitfiles - List changed files in branch
- [ ] blame - Smart git blame

**Search & Analysis:**
- [ ] quickgrep - Smart code search
- [ ] refs - Find all references
- [ ] unused - Find unused code

**Project Info:**
- [ ] stack - Detect tech stack
- [ ] readme - Generate README outline
- [ ] changelog - Smart changelog from commits

## 📦 Installation

Current binaries installed to: `~/bin/aspectmin/`
PATH configured in: `~/.zshrc`

## 🔄 Next Steps

1. Complete `--json` support for remaining 13 tools
2. Generate man pages for all tools
3. Create `aspectmin-llm-tools` repository
4. Implement LLM-focused tools
5. Add comprehensive testing
6. Create installation documentation

## 🎯 Goals

- **All tools** should support `--json` for machine-readable output
- **All tools** should have man pages
- **LLM tools** designed specifically for AI coding assistants
- **Clean separation** between general tools and LLM-specific tools
