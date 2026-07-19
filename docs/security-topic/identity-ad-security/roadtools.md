# ROADtools — Azure AD/Entra ID Exploration Toolkit

Python-based toolkit for exploring, analyzing, and attacking Azure AD (Entra ID) tenants. ROADrecon (the primary module) provides deep visibility into tenant object relationships beyond what Microsoft Graph typically exposes.

## How It Works

ROADtools consists of two main components:

| Component | Function |
|-----------|----------|
| **ROADrecon** | Tenant profiler — authenticates to Azure AD and enumerates users, groups, devices, applications, service principals, roles, policies, and trust relationships. Outputs to SQLite database. |
| **ROADtx** | Token acquisition and exchange — supports device code, OAuth 2.0, token manipulation, refresh token use, and Conditional Access policy evaluation bypass through resource-specific token requests. |
| **ROADlib** | Python library used by both above; provides Azure AD authentication primitives and Graph/AADGraph API wrappers. |

**ROADrecon data sources:**
- Azure AD Graph (`graph.windows.net`) — deprecated but still exposes richer data than MS Graph for some object types
- Microsoft Graph (`graph.microsoft.com`)
- MS Online PowerShell endpoints
- OAuth discovery / OpenID Connect metadata endpoints

**Attack-relevant data discovered:**
- Users with privileged roles (also via directory role assignments)
- Applications with high privileges (Application.ReadWrite.All, RoleManagement.ReadWrite.Directory)
- Service principal credentials (password/key credential dates)
- OAuth consent grants for third-party apps
- Conditional Access policy details
- Device registration and compliance state
- Hybrid Identity / AAD Connect configuration
- Tenant-level settings (allowed authentication methods, naming policy, external collaboration settings)

## Manual

### ROADrecon — Tenant Enumeration

```bash
# Interactive device code login (best)
roadrecon auth --device-code

# Username/password
roadrecon auth -u user@domain.com -p 'Password!'

# Username + interactive device code
roadrecon auth -u user@domain.com

# Azure CLI token
roadrecon auth --access-token (az account get-access-token --resource-type ms-graph).accessToken
```

### Data Collection

```bash
# Full enumeration (takes 30s–5min depending on tenant size)
roadrecon gather

# Specific modules
roadrecon gather --users
roadrecon gather --groups
roadrecon gather --apps
roadrecon gather --devices
roadrecon gather --policies
roadrecon gather --roles
roadrecon gather --all

# Only MS Graph (skip legacy AAD Graph)
roadrecon gather --use-msgraph
```

### Data Analysis

```bash
# Launch browser GUI (runs Flask locally)
roadrecon gui

# Default: http://localhost:5000
```

### CLI Queries

```bash
# Query the SQLite database directly
roadrecon query 'SELECT * FROM users WHERE dirsyncenabled = 1'

# List all users
roadrecon query 'SELECT displayName, userPrincipalName, accountEnabled FROM users'

# Privileged users
roadrecon query 'SELECT u.displayName, r.displayName
  FROM users u
  JOIN user_roles ur ON u.id = ur.user_id
  JOIN roles r ON ur.role_id = r.id
  WHERE r.displayName LIKE "%admin%"'
```

### ROADtx — Token Operations

```bash
# Device code login
roadtx devicecode -c <client-id> -r https://graph.microsoft.com

# Browser login
roadtx browserauth

# PRT (Primary Refresh Token) operations (Windows, logged-in only)
roadtx prt
roadtx prt -o prt.txt

# Token exchange
roadtx gettokens --prt prt.txt

# Cookie to token conversion
roadtx describe <cookie.txt
```

## Build

```bash
git clone https://github.com/dirkjanm/ROADtools.git
cd ROADtools
pip install .
```

## Install

```bash
# Pip
pip install roadrecon roadtx

# From source
pip install git+https://github.com/dirkjanm/ROADtools.git

# Virtualenv recommended
python -m venv roadtools
source roadtools/bin/activate
pip install roadrecon roadtx
```

## Package

| Manager | Command |
|---------|---------|
| pip | `pip install roadrecon roadtx` |
| Source | `pip install git+https://github.com/dirkjanm/ROADtools.git` |

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/dirkjanm/ROADtools |
| ROADtools docs | https://roadtools.readthedocs.io/ |
| Blog — ROADrecon guide | https://dirkjanm.io/introducing-roadtools-and-roadrecon-azure-ad-exploration-tool/ |
| Blog — ROADtx token operations | https://dirkjanm.io/roadtx-prt-initiation-and-sso-cookies/ |
| PyPI (roadrecon) | https://pypi.org/project/roadrecon/ |
| PyPI (roadtx) | https://pypi.org/project/roadtx/ |
