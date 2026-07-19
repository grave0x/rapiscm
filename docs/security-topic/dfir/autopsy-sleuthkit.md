# Autopsy / The Sleuth Kit — Digital Forensics Platform

## How It Works

The Sleuth Kit (TSK) is a C++ library and CLI toolset for forensic filesystem analysis. Autopsy is the Java GUI built on top of TSK, adding case management, ingest modules, and visualization.

**TSK architecture:**

- **File system layer** — raw image parsing (dd, E01, AFF, VMDK, VHD, RAW)
- **FS_Info** — abstraction over FAT, NTFS, ext2/3/4, HFS+, APFS, UFS, YAFFS2
- **Volume system** — MBR, GPT, BSD, Sun (VTOC), APM
- **Analysis tools:**
  - `mmls` — volume table (partition layout)
  - `fsstat` — filesystem details (cluster size, MFT location, journal)
  - `fls` — file listing (including deleted entries)
  - `icat` — file content by inode number
  - `istat` — inode/MFT entry metadata
  - `blkcat` / `blkstat` — raw block-level read/stat
  - `srch_strings` — strings extraction with encoding detection

**Autopsy additions:**
- **Case management** — per-case file tree, timeline, hash set, tag system
- **Ingest modules:** EXIF parsing, keyword search, hash lookup (NSRL), email/archive extraction, PhotoRec carving, Plaso timeline import
- **Graphical timeline** — event visualization across file creation/modification/access
- **Central repository** — multi-case hash db for cross-case correlation

## Manual

### Autopsy GUI

```bash
autopsy
# New Case → Case Name → Data Source Type (Disk Image / Local Folder)
# Select ingest modules → Finish
# Navigate: File Browser, Timeline, Keyword Search, Tagged Results
```

### TSK CLI Commands

```bash
# View partition layout
mmls evidence.dd

# Filesystem details
fsstat -o 2048 evidence.dd

# List files (including deleted) in a partition
fls -r -o 2048 evidence.dd

# List deleted files only
fls -r -d -o 2048 evidence.dd

# Get MFT entry details
istat -o 2048 evidence.dd 5678

# Extract file by inode
icat -o 2048 evidence.dd 5678 > recovered.bin

# Recover unallocated space blocks
blkcat -o 2048 evidence.dd 10000 20000 > unalloc.dd

# Global timeline
# (requires mactime format from fls)
fls -r -m / -o 2048 evidence.dd > body.txt
mactime -b body.txt > timeline.csv
```

### Ingest Modules (Autopsy)

| Module | Purpose |
|--------|---------|
| Recent Activity | LNK, Prefetch, Registry, Browser history |
| File Type Identification | Signature-based file category detection |
| Embedded File Extractor | OLE, ZIP, RAR extraction |
| Keyword Search | Regex/plain text across allocated + unallocated |
| Hash Lookup | MD5/SHA1 against NSRL, custom hash sets |
| Email Parser | PST, OST, MBOX extraction |
| EXIF Parser | JPEG metadata extraction |
| Plaso | Import super-timeline from log2timeline |
| PhotoRec | Carve deleted files by signature |

## Build

### The Sleuth Kit

```bash
git clone https://github.com/sleuthkit/sleuthkit.git
cd sleuthkit
./bootstrap && ./configure && make
sudo make install
```

### Autopsy

```bash
git clone https://github.com/sleuthkit/autopsy.git
cd autopsy
# Java project — open with NetBeans or build:
ant build
```

## Install

### The Sleuth Kit

```bash
# Linux
sudo apt install sleuthkit

# macOS
brew install sleuthkit

# Windows — download installer from GitHub releases
```

### Autopsy

```bash
# Download installer from autopsy.com
chmod +x autopsy-*.sh && ./autopsy-*.sh

# Or via package managers
# NOTE: Autopsy's ingest modules require the full installer, not just the binary
```

## Links

| Resource | URL |
|----------|-----|
| TSK GitHub | https://github.com/sleuthkit/sleuthkit |
| Autopsy GitHub | https://github.com/sleuthkit/autopsy |
| Autopsy official | https://www.autopsy.com/ |
| TSK docs | https://wiki.sleuthkit.org/ |
| Autopsy user guide | https://sleuthkit.org/autopsy/docs/user-docs/latest/ |
| Central repository | https://sleuthkit.org/autopsy/docs/user-docs/latest/centralRepository_page.html |
