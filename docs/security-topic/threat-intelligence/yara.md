# YARA — Pattern Matching Rule Engine

Pattern-matching engine for malware identification and classification. 10,000+ community rules. Identify malware families, suspicious files, and in-memory patterns.

## How It Works

YARA rules define pattern-matching logic against files, memory, or running processes. Rules combine string patterns (hex, text, regex) with boolean conditions and metadata. The engine scans across multiple files/processes efficiently.

**Rule anatomy:**
- **meta** — rule metadata (author, description, hash)
- **strings** — definitions of hex/text/regex patterns
- **condition** — boolean logic combining strings (all of them, 2 of them, $s1 and not $s2)

## Manual

```bash
# Scan file
yara rule.yar target_file

# Scan directory recursively
yara rule.yar /path/to/directory

# Scan process memory (Linux)
yara rule.yar <PID>

# Scan all running processes
yara -p rule.yar

# Print matching strings
yara -s rule.yar target_file

# Count matches
yara -c rule.yar target_file

# External variables
yara -d option=1 rule.yar target_file

# Scan with timeout (seconds)
yara -t 30 rule.yar target_file
```

### Rule Examples

```yara
rule Ransomware_Notice
{
    meta:
        description = "Detects ransomware ransom note files"
        author = "Threat Intel Team"
        date = "2024-01-01"

    strings:
        $s1 = "your files have been encrypted" nocase
        $s2 = "decrypt" nocase
        $s3 = "bitcoin" nocase
        $s4 = /[A-Za-z0-9]{32,}\.(onion|html|txt)/

    condition:
        all of ($s1, $s2) and $s3 and #s4 >= 2
}

rule Mimikatz_DLL
{
    meta:
        description = "Detects Mimikatz DLL in memory"
        reference = "https://github.com/gentilkiwi/mimikatz"

    strings:
        $mimi1 = "mimidrv" fullword wide
        $mimi2 = "kiwi" fullword wide
        $mimi3 = "sekurlsa" fullword wide

    condition:
        any of them
}
```

### Programmatic (yara-python)

```python
import yara

# Compile rules
rules = yara.compile(filepath='rules.yar')

# Scan file
matches = rules.match('/path/to/file.exe')
for match in matches:
    print(match.rule, match.tags, match.strings)

# Scan raw data
matches = rules.match(data=open('file.bin', 'rb').read())

# Scan process
matches = rules.match(pid=1234)
```

## Build

```bash
git clone https://github.com/VirusTotal/yara.git
cd yara
./bootstrap.sh
./configure --enable-cuckoo --enable-magic --enable-profiling
make -j$(nproc)
sudo make install
```

## Install

```bash
# Linux (source)
# See Build section above

# macOS
brew install yara

# Windows
# Download installer from GitHub releases

# Python bindings
pip install yara-python
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/VirusTotal/yara |
| Docs | https://yara.readthedocs.io/ |
| Community rules | https://github.com/YARA-Rules/rules |
| yara-python | https://github.com/VirusTotal/yara-python |
| Valhalla (YARA feed) | https://valhalla.nextron-systems.com/ |
