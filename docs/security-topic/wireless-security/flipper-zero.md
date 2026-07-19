# Flipper Zero — Multi-Tool Portable Pentesting Device

Handheld multi-tool for pentesters, security researchers, and hardware hackers. Sub-GHz, NFC, RFID, iButton, Infrared, GPIO, Bluetooth, and USB BadUSB capabilities.

## How It Works

Flipper Zero runs custom firmware (original or community forks like Unleashed, Momentum, Xtreme). Interaction via 5-button D-pad, 1.4" monochrome LCD, and USB-C.

**Hardware capabilities:**

| Mode | Frequency/Protocol | Range |
|------|-------------------|-------|
| Sub-GHz | 300–928 MHz (regional) | Up to 50m |
| NFC | 13.56 MHz (Type A/B, FeliCa) | Contact |
| 125kHz RFID | EM4100, HID Prox, Indala | Contact |
| iButton | Dallas 1-Wire (DS1990A, etc.) | Contact |
| Infrared | 38kHz (RC5, NEC, Samsung, Sony) | Up to 10m |
| GPIO | 3.3V logic (UART, I2C, SPI, PWM) | Pin header |
| Bluetooth | BLE (remote control, mobile app) | 10m |
| USB | BadUSB (HID keyboard injection) | Wired |

**Key attack scenarios:**
- Clone/emulate access cards (RFID, NFC, iButton)
- Garage/gate replay attacks (Sub-GHz capture + replay)
- IR remote cloning (TVs, projectors, IR-controlled doors)
- BadUSB payload injection (Rubber Ducky scripts)
- GPIO debugging (UART console access on embedded devices)
- BLE spam (Apple, Android notification floods)

## Manual

```bash
# CLI over USB
flipper-cli device_info
flipper-cli storage list /int

# Run Sub-GHz capture
# Navigate: Sub-GHz -> Read -> RAW

# BadUSB payload
# Copy duckyscript.txt to /ext/badusb, run on connected device

# GPIO UART (115200 baud)
# Connect pins: TX/RX/GND, open CLI: gpio uart -b 115200
```

## Install

```bash
# Official firmware (qFlipper)
# Download qFlipper from flipperzero.one
# USB flash via GUI

# Custom firmware (Unleashed)
# Download from github.com/DarkFlippers/unleashed-firmware
# Use qFlipper to flash .dfu file

# Build from source
git clone https://github.com/flipperdevices/flipperzero-firmware.git
cd flipperzero-firmware
./fbt
./fbt flash_usb
```

## Build

```bash
git clone --recursive https://github.com/flipperdevices/flipperzero-firmware.git
cd flipperzero-firmware
./fbt DEBUG=1
# Output: build/f7-firmware-D/
# Flash: ./fbt flash_usb
```

## Package

Official firmware via qFlipper installer (Linux/macOS/Windows). Community firmware: Unleashed, Momentum, Xtreme — GitHub releases. FAP (Flipper Application Package) store: https://lab.flipper.net/apps.

## Links

| Resource | URL |
|----------|-----|
| Official site | https://flipperzero.one/ |
| GitHub (official) | https://github.com/flipperdevices/flipperzero-firmware |
| Docs | https://docs.flipperzero.one/ |
| qFlipper | https://flipperzero.one/update |
| Unleashed firmware | https://github.com/DarkFlippers/unleashed-firmware |
| Awesome Flipper | https://github.com/djsime1/awesome-flipperzero |
| Mobile app | https://flipperzero.one/downloads |
| FAP Store | https://lab.flipper.net/apps |
