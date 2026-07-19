# Wireless Security Testing

Assess WiFi, Bluetooth/BLE, RFID/NFC, and SDR-based attack surfaces. Covers capture, cracking, exploitation, and physical-layer testing.

## Sub-Topics

| Topic | Description |
|-------|-------------|
| **WiFi/WLAN Assessment** | WPA2/WPA3 handshake capture & cracking, PMKID attack, WPS PIN brute-force, evil twin / rogue AP, deauth flooding, PKMID, WPA3 SAE weaknesses, 802.11w/802.11ax testing |
| **Bluetooth / BLE** | GATT service enumeration, BT sniffing (Ubertooth, nRF52), BLE MITM, MAC address tracking, BT Classic PIN cracking, BLE reconnection attacks, BLUR/BLUE attacks |
| **RFID / NFC** | 125 kHz + 13.56 MHz card reading/dumping/cloning, MIFARE Classic crypto-1 cracking (nested/darkside/hardnested), EM4100/HID Prox/Indala cloning, NDEF parsing, HCE emulation |
| **SDR / RF Analysis** | Wideband spectrum analysis, signal demodulation, protocol reverse-engineering, frequency-hopping analysis, ADS-B/GSM/LoRa/ZigBee analysis |
| **Drone Security** | RF-based drone detection, drone model fingerprinting via RF signatures, MAVLink protocol analysis, payload detection via micro-Doppler |
| **Wireless IDS/IPS** | Rogue AP detection, deauth detection, evil twin identification, spectrum anomaly detection, WIDS rules |
| **Physical Layer** | Signal jamming (authorized), GPS spoofing detection, replay attacks, relay attacks, voltage glitching, side-channel |

## Methods

1. **Reconnaissance** — Wardriving, spectrum scan, beacon/probe analysis, channel discovery, WPS enumeration
2. **Capture** — Handshake capture (4-way), PMKID capture, BLE advertising capture, SDR IQ capture
3. **Cracking** — Dictionary/rule-based WPA2 PSK cracking (hashcat/aircrack), PMKID offline crack, MIFARE key recovery
4. **Exploitation** — Evil twin deployment, KARMA attack, BLE MITM, RFID cloning, replay attack, drone signal injection
5. **Post-Exploitation** — Network pivot from connected clients, lateral movement assessment, persistence evaluation

## Tool Comparison

| Tool | Category | License | Key Strength |
|------|----------|---------|--------------|
| **aircrack-ng** | WiFi cracking | GPLv2 | Full WiFi suite: capture, inject, crack |
| **hashcat** | Password cracking | MIT | GPU-accelerated, supports WPA2 PMKID/handshake |
| **Bettercap** | MITM framework | GPLv3 | Modular: WiFi/BLE/HTTP(S)/DNS spoofing |
| **Kismet** | Wireless IDS | GPLv2 | Multi-protocol device detection, logging, alerting |
| **Reaver** | WPS attack | GPLv2 | WPS PIN brute-force against WPS-enabled APs |
| **Proxmark3** | RFID research | GPLv2+ | Read/dump/clone/emulate LF+HF, crypto-1 cracking |
| **GNU Radio** | SDR framework | GPLv3 | Flowgraph-based signal processing/demodulation |
| **HackRF One** | SDR platform | GPLv2 | 1 MHz–6 GHz RX/TX, wideband capture, signal injection |
| **Flipper Zero** | Multi-tool | Commercial | RFID/NFC/iButton/Infrared/BadUSB/GPIO analysis |
| **Wifiphisher** | Evil twin | MIT | Captive portal + evil twin phishing framework |

## Tool Docs

| File | Tool |
|------|------|
| [aircrack-ng.md](aircrack-ng.md) | aircrack-ng |
| [hashcat.md](hashcat.md) | hashcat |
| [Bettercap.md](Bettercap.md) | Bettercap |
| [Kismet.md](Kismet.md) | Kismet |
| [Reaver.md](Reaver.md) | Reaver |
