# Reverse Engineering

Tools for disassembly, decompilation, binary analysis, and debugging across multiple architectures and formats.

## Topics

| Topic | Description |
|-------|-------------|
| Static Analysis | Examine binary without execution — control-flow graphs, imports/exports, strings, cross-references, file format parsing |
| Dynamic Analysis | Observe runtime behavior — register/memory state, API calls, debug traces, network flows |
| Symbolic Execution | Treat inputs as symbolic values, explore all feasible execution paths (angr) |
| Binary Diffing | Compare two binary versions to locate patches/vulnerabilities (BinDiff, Diaphora) |
| Mobile RE | Decompile DEX/APK to Java (JADX); reverse native .so (Ghidra/IDA); instrument at runtime (Frida) |
| .NET RE | Assembly browsing and C# decompilation (ILSpy, dnSpyEx) |
| Firmware RE | Extract filesystems (binwalk), analyze bootloaders, kernel images, cryptographic material |

## Methods

- **Static Analysis:** IDA Pro / Ghidra / Binary Ninja / radare2 — file format parsing, CFG/DFG generation, string search, XREFs
- **Dynamic Analysis:** x64dbg / GDB / WinDbg — breakpoints, memory dumps, register tracing, step-through debugging
- **Instrumentation:** Frida — inject JS hooks into running processes across platforms
- **Emulation:** Unicorn — lightweight CPU emulation for symbolic/concolic execution
- **Hybrid Decompilation:** Combine static decompilation with dynamic analysis data for improved pseudocode
- **Binary Diffing:** BinDiff / Diaphora — structural diffing of functions, basic blocks, and call graphs

## Tools

| Tool | Category | Description | License |
|------|----------|-------------|---------|
| IDA Pro | Disassembler + Decompiler | Industry-standard disassembler, Hex-Rays decompiler, 50+ architectures | Commercial (~$1k+) |
| Ghidra | RE Framework | NSA open-source RE framework with decompiler, 50+ processor families, collaboration server | Apache 2.0 |
| Binary Ninja | RE Platform | Modern RE with HLIL decompiler, fast native UI, Python/C++/Rust API | Commercial ($299-$1,499) |
| radare2 / Rizin | RE Framework | CLI framework with disassembly, debugging, scripting; Cutter Qt GUI | LGPL / GPL |
| x64dbg | Debugger | Open-source Windows debugger for 32/64-bit user-mode apps | GPL 3.0 |
| Frida | Instrumentation | Cross-platform dynamic instrumentation — inject JS hooks into running processes | wxWindows Library |
| JADX | Mobile RE | DEX/APK to Java decompiler for Android reverse engineering | Apache 2.0 |
| ILSpy | .NET RE | Open-source .NET assembly browser and C# decompiler | MIT |
| dnSpyEx | .NET RE | .NET debugger + decompiler, community fork of dnSpy | GPL 3.0 |
| Capstone | Disassembly Lib | Lightweight multi-architecture disassembly library | BSD |
| Keystone | Assembly Lib | Multi-architecture assembler library | GPL 2.0 |
| Unicorn | Emulation | Lightweight multi-architecture CPU emulation framework | GPL 2.0 |
| angr | Binary Analysis | Python binary analysis framework with symbolic execution for vulnerability discovery | BSD 2-Clause |
| BinDiff | Binary Diffing | Binary diffing tool for comparing executables and identifying patched code | Apache 2.0 |
| DiE | Packer Detection | File type, compiler, and packer identification via signatures + heuristics | MIT |
