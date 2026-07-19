# NoSQLMap — NoSQL Injection & Enumeration Tool

Open-source tool for automated NoSQL database security testing. Targets MongoDB, CouchDB, and other NoSQL backends. Injects, enumerates, and extracts data.

## How It Works

NoSQLMap automates NoSQL injection detection and exploitation. Operates in interactive menu mode or via command-line options.

**Attack modes:**

| Mode | Description |
|------|-------------|
| **Authentication Bypass** | Circumvent login via NoSQL injection in JSON/URL parameters |
| **Data Extraction** | Dump databases, collections, documents via injection |
| **Blind Injection** | True/false inference and time-based blind NoSQL injection |
| **Server-Side Injection** | JavaScript injection for MongoDB (where clause abuse) |
| **Enumeration** | Discover NoSQL databases and accessible collections |

**Supported databases:** MongoDB (primary), CouchDB, MySQL (NoSQL features).

**Injection vectors:** URL parameters, JSON body, HTTP headers, REST API parameters.

## Manual

```bash
# Interactive menu
python nosqlmap.py

# Direct injection test
python nosqlmap.py --url "http://target.com/api/login" \
  --injectable "username" --technique U

# Authentication bypass
python nosqlmap.py --url "http://target.com/api/login" \
  --data '{"username":"admin","password":"test"}' \
  --auth-bypass

# Enumerate databases (requires injection point)
python nosqlmap.py --url "http://target.com/api/users" \
  --technique U --dbs

# Blind injection detection
python nosqlmap.py --url "http://target.com/api/search" \
  --param "q" --technique B

# Custom headers
python nosqlmap.py --url "http://target.com/api/data" \
  --headers "Authorization: Bearer <token>"
```

## Build

```bash
git clone https://github.com/codingo/NoSQLMap.git
cd NoSQLMap
python3 setup.py install
```

## Install

```bash
# From pip (if available)
pip3 install nosqlmap

# From source (recommended)
git clone https://github.com/codingo/NoSQLMap.git
cd NoSQLMap
pip3 install -r requirements.txt
python3 setup.py install

# Kali
sudo apt install nosqlmap
```

## Package

| Manager | Command |
|---------|---------|
| pip | `pip install nosqlmap` |
| apt | `sudo apt install nosqlmap` |
| Source | GitHub releases |

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/codingo/NoSQLMap |
| Docs | https://github.com/codingo/NoSQLMap/wiki |
| OWASP NoSQL | https://owasp.org/www-project-web-security-testing-guide/.../Testing_for_NoSQL_Injection |
| MongoDB injection | https://github.com/codingo/NoSQLMap/wiki/MongoDB-Injection-Basics |
