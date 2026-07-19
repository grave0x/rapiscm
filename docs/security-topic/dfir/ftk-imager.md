# FTK Imager — Forensic Imaging & Preview Tool

## How It Works

FTK Imager (by AccessData) is a lightweight forensic imaging tool for creating bit-for-bit copies (forensic images) of drives, memory, and individual files. Also used for quick triage preview of forensic images.

**Key capabilities:**

- **Imaging:**
  - Create forensic images in: DD (raw), E01 (EnCase), AFF, SMART
  - Physical drive, logical partition, folder contents, or live memory (RAM)
  - Verify images with MD5/SHA1 hash — acquisition hash vs post-verify hash
  - Segmentation: split images into configurable chunk sizes (650 MB, 2 GB, 4 GB, etc.)
  - Compression: E01 supports segment-level compression
- **Mounting:** Mount forensic images as read-only virtual drives for third-party tool access
- **Preview:** Browse filesystem, view file contents, export artifacts — even from live systems
- **Memory acquisition:** Capture physical RAM (Windows) for volatile evidence preservation
- **Registry viewer:** Read-only registry hive access (Windows)

**Use cases:**
- Rapid triage: preview filesystem without full analysis setup
- Memory acquisition: capture RAM before shutdown
- Image verification: validate acquisition integrity before shipping/archiving
- Evidence extraction: export specific files from forensic images without loading into full forensic suite

## Manual

### GUI

```bash
# Launch
FTK Imager.exe

# Create image:
# File → Create Disk Image →
#   Source: Physical Drive / Logical Drive / Image File / Contents of Folder
#   Destination: Image Type (DD/E01/AFF/SMART)
#   Evidence Item Information: Case Number, Evidence Number, Examiner, Notes
#   Image Destination: Folder + filename
#   → Verify images after creation (recommended)
#   → Pre-calculate Progress Statistics
#   → Start

# Mount image: File → Image Mounting → select image → read-only

# Memory acquisition: File → Capture Memory → destination + filename
```

### CLI (Command Line)

```bash
# Command-line options limited — primarily GUI tool.
# For CLI imaging, use: ftkimager.exe
ftkimager.exe \\.\PHYSICALDRIVE0 image.e01 --e01 --verify
```

### Key Workflows

```bash
# 1. Physical memory acquisition (triage)
# File → Capture Memory → select output path
# Produces: <case>.mem (raw RAM) + <case>.mempage (page file snapshot)

# 2. Quick preview of image content
# File → Add Evidence Item → Image File → browse to .E01/.dd/.AFF
# Browse tree: expand folder → view files → Export (right-click)

# 3. Export files from image without mounting
# Right-click file/folder → Export Files → destination

# 4. Hash verification
# File → Verify Drive/Image → select image → MD5/SHA1/CRC32
# Compares: stored acquisition hash vs computed hash from image
```

### Export Options

```bash
# Export File Hash List (for all files in image)
# Tools → Export File Hash List → CSV of all filenames + hashes

# Export Directory Listing
# Tools → Export Directory Listing → CSV of folder structure
```

## Install

```bash
# Download from AccessData website (free tool, registration required)
# https://www.exterro.com/ftk-imager

# Windows: run installer .exe
# Linux: no native version — use WINE or alternative (Guymager)
# macOS: no native version

# Portable version available (no install required)
```

## Links

| Resource | URL |
|----------|-----|
| Download | https://www.exterro.com/ftk-imager |
| Product page | https://www.exterro.com/digital-forensics-software/ftk-imager |
| User guide | https://www.exterro.com/support/ftk-imager |
| AccessData support | https://support.exterro.com/ |
