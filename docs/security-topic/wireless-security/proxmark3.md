# Proxmark3 — RFID/NFC Security Research Tool

Multi-frequency RFID/NFC reader/writer/sniffer. Supports low-frequency (125kHz) and high-frequency (13.56MHz) tags. Used for cloning, cracking, and analyzing contactless cards.

## How It Works

Proxmark3 interacts with RFID/NFC fields via antenna. Operation modes:

- **LF (125kHz)** — EM4102, T55xx, Hitag, Indala, etc. Read/Write/Simulate/clone.
- **HF (13.56MHz)** — Mifare Classic/Plus/Desfire, NFC Type 2/3/4, iClass, etc.
- **Standalone mode** — Flash custom firmware, run scripts without PC tether.

**Key operations:**
- `lf search` — Auto-detect LF tag type
- `hf mf` — Mifare family: nested authentication, hardnested, Darkside attack
- `hf mf chk` — Default key check (keys stored in dictionaries)
- `hf mf restore` — Clone Mifare dump to blank card
- `hf iclass` — iClass tag read/write/sim
- `hf 14a sniff` — Sniff HF communication between reader and tag

## Manual

```bash
# Enter Proxmark3 client
proxmark3 /dev/ttyACM0

# Search for LF tags
lf search

# Read Mifare 1K (all sectors, default keys)
hf mf autopwn

# Dump Mifare to file
hf mf dump --dump f:\dumps\card1

# Restore dump to blank card
hf mf restore --dump f:\dumps\card1

# Crack Mifare with nested attack
hf mf nested --dictionary keys.dic

# Sniff HF reader-tag communication
hf sniff

# iClass read
hf iclass readblk --blk 0
```

## Build

```bash
git clone https://github.com/RfidResearchGroup/proxmark3.git
cd proxmark3
make clean && make -j
# Flashes firmware to device
make flash
```

## Install

```bash
# Debian/Ubuntu
sudo apt install proxmark3 proxmark3-client
# Or from source (see Build)

# macOS
brew install proxmark3

# Compile from source for latest features
# Requires: arm-none-eabi-gcc, libreadline, pcre, qt5 (for GUI)
```

## Package

Available in Kali (`proxmark3`), Debian/Ubuntu repos. Latest firmware via GitHub releases. Community builds on Arch AUR (`proxmark3-git`).

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/RfidResearchGroup/proxmark3 |
| Official site | https://www.proxmark.com/ |
| Docs | https://github.com/RfidResearchGroup/proxmark3/wiki |
| Iceman fork | https://github.com/RfidResearchGroup/proxmark3 |
| Proxmark3 forums | https://www.proxmark.com/forum |
| Awesome Proxmark3 | https://github.com/coopstools/awesome-proxmark3 |
