# HackRF One — Software-Defined Radio Peripheral

1 MHz–6 GHz SDR platform from Great Scott Gadgets. Half-duplex transceiver — TX and RX capability. USB-powered, open-source hardware. Primary tool for RF analysis, signal replay, and protocol reverse engineering.

## How It Works

HackRF is a USB-attached SDR. Host software controls tuning, gain, sample rate, and data streaming.

**Specs:**
- Frequency range: 1 MHz – 6 GHz
- Sample rate: 20 Msps (8-bit quadrature samples)
- Bandwidth: ~20 MHz (limited by USB 2.0)
- Half-duplex: TX or RX, not simultaneously
- Antenna port: SMA female

**Common use cases:**
- **Signal capture/replay** — Record RF samples, retransmit (key fobs, garage openers, remotes)
- **Spectrum analysis** — Sweep wide bands with `hackrf_sweep`
- **Protocol analysis** — Decode ISM band protocols (433MHz, 915MHz, 2.4GHz)
- **GSM/LTE monitoring** — With gr-gsm and OpenBTS
- **GPS spoofing** — With gps-sdr-sim

## Manual

```bash
# List devices
hackrf_info

# Spectrum sweep (1–2 GHz)
hackrf_sweep -f 1000000000 -l 2000000000 -w 10000000

# Capture 10 seconds at 915 MHz
hackrf_transfer -r capture.bin -f 915000000 -s 20000000 -n 200000000

# Replay captured signal
hackrf_transfer -t capture.bin -f 915000000 -s 20000000 -x 20

# Receive and pipe to GQRX-style app
hackrf_transfer -r /dev/stdout -f 433920000 -s 2000000 | ...

# Check firmware version
hackrf_clock -f
hackrf_cpld -x
```

## Build

```bash
# Firmware (requires ARM toolchain)
git clone https://github.com/greatscottgadgets/hackrf.git
cd hackrf/firmware
make

# Host tools
cd hackrf/host
mkdir build && cd build
cmake ..
make && sudo make install
```

## Install

```bash
# Debian/Ubuntu
sudo apt install hackrf libhackrf-dev
# Also install gr-osmosdr for GNU Radio integration

# macOS
brew install hackrf

# Windows
# Install Zadig driver, then use official release binaries

# Docker
docker pull greatscottgadgets/hackrf:latest
```

## Package

| Manager | Command |
|---------|---------|
| apt | `sudo apt install hackrf` |
| Homebrew | `brew install hackrf` |
| Kali | pre-installed (`hackrf`) |
| Source | GitHub releases with prebuilt binaries |

## Links

| Resource | URL |
|----------|-----|
| Official site | https://greatscottgadgets.com/hackrf/ |
| GitHub | https://github.com/greatscottgadgets/hackrf |
| Docs | https://hackrf.readthedocs.io/ |
| Getting started | https://greatscottgadgets.com/hackrf/one/ |
| Awesome HackRF | https://github.com/fate0/awesome-hackrf |
| Osmocom blog | https://osmocom.org/projects/hackrf/wiki |
