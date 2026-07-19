# ILSpy / dnSpy — .NET Decompiler & Debugger

## How It Works

ILSpy is the reference .NET decompiler (open-source, OSS); dnSpy wraps ILSpy's engine with a full debugger and editor for live .NET binary modification.

**ILSpy engine architecture:**
- **ICSharpCode.Decompiler** — C# IL → C#/VB decompilation pipeline
  - Read PE metadata (ECMA-335) → token tables, member refs, generic params
  - Build symbol tree (namespaces → types → methods → locals)
  - IL instruction decoding → AST transformation (expression trees, async/await)
  - Language-specific output: C#, VB.NET, IL (raw opcodes)
- **Plugins** — extensible via `MEF` (Managed Extensibility Framework), assemblies dropped in `Plugins/`

**dnSpy additions:**
- **Debugger** — MDIL debugging engine: attach to .NET Framework/Core processes
- **Editor** — modify C# source in decompiled view → recompile with Roslyn → patch assembly on disk
- **BAML decompiler** — WPF XAML binary → readable XAML
- **Hex editor** — byte-level editing overlay
- **Export** — project format (.csproj) for rebuild

## Manual

### Launch

```bash
# ILSpy GUI
ilspy

# ILSpy CLI (cmd)
ilspyc.exe assembly.dll -o output_dir

# dnSpy
dnspy
```

### ILSpy CLI Commands

```bash
# Decompile to single file
ilspycmd assembly.dll

# Decompile to project directory
ilspycmd -p assembly.dll -o ./src

# List types
ilspycmd -l assembly.dll

# Decompile with IL (raw opcodes)
ilspycmd -il assembly.dll
```

### dnSpy Features

```bash
# Attach to process
# Debug → Attach to Process → select .NET process → break / step

# Edit method
# Right-click method → Edit Method (C#) → modify → Compile
# dnSpy patches IL in memory (or save to assembly)

# Search assemblies
# Ctrl+Shift+K — search across all loaded modules
# Ctrl+D — find references to symbol
```

## Build

```bash
# ILSpy
git clone https://github.com/icsharpcode/ILSpy.git
cd ILSpy
dotnet build

# dnSpy (legacy — no longer maintained upstream)
git clone https://github.com/dnSpy/dnSpy.git
cd dnSpy
dotnet build
```

## Install

### ILSpy

```bash
# Chocolatey
choco install ilspy

# Scoop
scoop install ilspy

# Or download from GitHub releases
```

### dnSpy

```bash
# Download from GitHub releases
wget https://github.com/dnSpy/dnSpy/releases/download/v6.1.8/dnSpy-net-win64.zip
unzip dnSpy-net-win64.zip -d dnSpy

# Chocolatey
choco install dnspy
```

## Links

| Resource | URL |
|----------|-----|
| ILSpy GitHub | https://github.com/icsharpcode/ILSpy |
| ILSpy releases | https://github.com/icsharpcode/ILSpy/releases |
| dnSpy GitHub | https://github.com/dnSpy/dnSpy |
| dnSpy releases | https://github.com/dnSpy/dnSpy/releases |
| dnSpyEx (community fork) | https://github.com/dnSpyEx/dnSpy |
