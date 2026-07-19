# x64dbg — Windows Debugger

## How It Works

x64dbg is an open-source GUI debugger for Windows 32-bit (x32dbg) and 64-bit (x64dbg) user-mode applications. It combines OllyDbg-like UI with modern plugin architecture and scripting.

**Key features:**
- **Dump** — memory hex view with structure parsing, highlighting, search
- **Stack** — call stack with SEH, arguments, return addresses
- **Disassembly** — multi-tab, follow expressions, conditional breakpoints
- **Graph view** — control-flow graph with blocks, XREFs, annotations
- **Symbols** — import/export table, TLS callbacks, section headers
- **TitanEngine** — underlying debugging engine with process manipulation, breakpoints, memory patching
- **Plugin system** — C/C++/Python (via bridge.py), 100+ community plugins
- **Scripting** — x64dbg script language (similar to OllyScript) + Python bridge

**Debugging methods:**
- INT3 breakpoints, hardware breakpoints (DR0-DR3), memory breakpoints
- Conditional breakpoints with logging/auto-pause
- Run tracing + instruction counting for anti-debug bypass
- ScyllaHide plugin — hide debugger from common anti-debug checks

## Manual

### Launch

```bash
x64dbg\release\x64\x64dbg.exe    # 64-bit debugger
x64dbg\release\x32\x32dbg.exe    # 32-bit debugger
```

File → Open → select .exe / attach to running process.

### Common Commands

| Action | Shortcut |
|--------|----------|
| Run | F9 |
| Step into | F7 |
| Step over | F8 |
| Execute till return | Ctrl+F8 |
| Toggle breakpoint | F2 |
| Conditional breakpoint | Shift+F2 (expression, log, cond, auto-pause) |
| Follow in dump | Right-click → Follow in dump |
| Search for strings | Ctrl+E (every sequence) / Ctrl+F (in active window) |
| Graph mode | G |
| Show call stack | View → Call stack / Alt+K |

### ScyllaHide Configuration

```bash
# Plugins → ScyllaHide → Options
# Profile: select target process (default, VMWare, VirtualBox, etc.)
# Hooks: NtQueryInformationProcess, NtClose, NtSetInformationThread, etc.
```

## Build

```bash
git clone --recursive https://github.com/x64dbg/x64dbg.git
cd x64dbg
cmake -G "Visual Studio 17 2022" -A x64 -B build
cmake --build build --config Release
```

## Install

```bash
# Download pre-built package from github.com/x64dbg/x64dbg/releases
# Extract to desired location, run x64dbg.exe

# Or via scoop
scoop bucket add extras
scoop install x64dbg

# Chocolatey
choco install x64dbg
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://x64dbg.com/ |
| GitHub | https://github.com/x64dbg/x64dbg |
| Blog | https://blog.x64dbg.com/ |
| Plugin list | https://github.com/x64dbg/x64dbg/wiki/Plugins |
| ScyllaHide | https://github.com/x64dbg/ScyllaHide |
