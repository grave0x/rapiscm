# hashcat — GPU-Accelerated Password Recovery

World's fastest password recovery tool. GPU-accelerated cracking for 300+ hash types including WPA2 PMKID/handshake, NTLM, bcrypt, SHA, and raw kernel modes.

## How It Works

Hash modes (`-m`) target specific algorithm types. Attack modes (`-a`) define strategy:

| Attack Mode | ID | Description |
|-------------|----|-------------|
| Straight | 0 | Dictionary — each word from wordlist |
| Combination | 1 | Wordlist × wordlist concatenation |
| Brute-force | 3 | Mask attack (charset per position) |
| Hybrid dict + mask | 6 | Wordlist + mask suffix |
| Hybrid mask + dict | 7 | Mask prefix + wordlist |
| Association | 9 | Crack one hash from another's plaintext |

**Wireless modes:** `-m 22000` (WPA-PBKDF2-PMKID+EAPOL), `-m 16800` (WPA-PMKID-PBKDF2).

## Manual

```bash
# WPA2 handshake/PMKID (convert .cap/hccapx to .22000 first)
hcxpcapngtool -o hash.22000 capture.cap
hashcat -m 22000 hash.22000 wordlist.txt

# NTLMv2
hashcat -m 5600 hash.txt wordlist.txt

# Mask attack (8-char lowercase)
hashcat -m 22000 hash.22000 -a 3 ?l?l?l?l?l?l?l?l

# Rules-based cracking
hashcat -m 22000 hash.22000 wordlist.txt -r best64.rule

# Show cracked hashes
hashcat -m 22000 hash.22000 --show
```

## Build

```bash
git clone https://github.com/hashcat/hashcat.git
cd hashcat
make -j$(nproc)
sudo make install
```

## Install

```bash
# Debian/Ubuntu
sudo apt install hashcat

# macOS
brew install hashcat

# Arch
sudo pacman -S hashcat

# Docker
docker pull hashcat/hashcat

# Pre-built binary from GitHub releases
```

## Package

GitHub releases provide prebuilt binaries for Linux (glibc), macOS, and Windows. Also distributed in Kali Linux.

## Links

| Resource | URL |
|----------|-----|
| Official site | https://hashcat.net/hashcat/ |
| GitHub | https://github.com/hashcat/hashcat |
| Wiki | https://hashcat.net/wiki/ |
| Example hashes | https://hashcat.net/wiki/doku.php?id=example_hashes |
| Rule files | https://github.com/hashcat/hashcat/tree/master/rules |
| hashcat-utils | https://github.com/hashcat/hashcat-utils |
