# Plaso (log2timeline) — Super-Timeline Engine

## How It Works

Plaso (formerly log2timeline) ingests forensic artifacts (filesystem, registry, event logs, web history, etc.) and generates a unified super-timeline — every timestamped event sorted chronologically with full context.

**Key architecture:**
- **Parser system** — 200+ multi-format parsers for registry hives, EVTX, PLF, SQLite, $MFT, $J, $UsnJrnl, Prefetch, Browser data, Mac plists, Linux logs
- **Multi-process** — worker pool-based parallel parsing. One collector feeds queues; multiple workers consume and produce.
- **Storage** — pluggable storage backends (SQLite, PostgreSQL, Elasticsearch)
- **Output** — CSV, JSON, Elastic, Timesketch-compatible format
- **Pre-processing** — file-type identification, archive extraction (zip/7z), VSS (Volume Shadow Copy) iteration

**Pipeline:**
1. Image mount / directory path → file system iteration
2. File-type identification → route to appropriate parser modules
3. Multi-threaded parsing → extract all timestamped events
4. Deduplication + event aggregation
5. Storage write → CSV or Timesketch import

## Manual

### Launch

```bash
# Create timeline from disk image
log2timeline --storage-file case.plaso image.E01

# From directory (live system or mounted image)
log2timeline --storage-file case.plaso /mnt/case/C:/

# Pick parsers to run
log2timeline --parsers "filestat,winreg,evtx,prefetch" --storage-file case.plaso /mnt/case/

# List available parsers
log2timeline --parser-list
```

### Output

```bash
# Export to CSV
psort -o l2tcsv case.plaso > timeline.csv

# Filter by date range
psort -o dynamic -q "date > '2026-01-01' AND date < '2026-02-01'" case.plaso

# Timeline summary
psort -o null case.plaso -q "date > '2026-05-01'" --slice 50
```

### Timesketch Integration

```bash
# Export for Timesketch
psort -o timesketch case.plaso > case.tsk.csv

# Import into Timesketch via CLI
timesketch_importer.py --sketch <ID> case.tsk.csv
```

### Common Filters

```bash
# Filter by parser
psort -o dynamic case.plaso --analysis "tagging" -q "parser == 'prefetch'"

# Filter by event type
psort -o dynamic case.plaso -q "data_type == 'windows:evtx:record'"

# Source filter
psort -o dynamic case.plaso -q "source == 'Registry'"
```

## Build

```bash
git clone https://github.com/log2timeline/plaso.git
cd plaso
pip install -r requirements.txt
pip install -e .
```

## Install

```bash
# Option 1 — pip
pip install plaso

# Option 2 — Docker
docker pull log2timeline/plaso:latest
docker run --rm -v $PWD:/data log2timeline/plaso log2timeline \
  --storage-file /data/case.plaso /data/image.E01

# Option 3 — Ubuntu PPA
sudo add-apt-repository ppa:gift/stable
sudo apt update && sudo apt install plaso-tools
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/log2timeline/plaso |
| Docs | https://plaso.readthedocs.io/ |
| Parser overview | https://plaso.readthedocs.io/en/latest/sources/user/Parsers.html |
| Timesketch | https://timesketch.org/ |
