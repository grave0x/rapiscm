# AzureHound — Azure/Entra ID Enumeration for BloodHound

Go-based tool that collects Azure AD (Entra ID) and Azure RBAC data and converts it into BloodHound-compatible graph output. Maps attack paths in hybrid and cloud-only identity environments.

## How It Works

AzureHound authenticates to Microsoft Graph (`graph.microsoft.com`) and Azure Resource Management (`management.azure.com`) APIs to enumerate:

- Users, groups, roles, service principals, applications
- Role assignments (Azure AD roles + Azure RBAC)
- Group memberships (transitive + non-transitive)
- Device registrations
- Conditional Access policies
- Tenant relationships
- Subscription, management group, resource group hierarchy
- Key Vaults, VMs, storage accounts, and other Azure resources

**Output:** JSON files — nodes (users, groups, apps, roles, subscriptions) and edges (assignments, memberships, privileges). Directly ingestible by BloodHound CE.

**Attack path primitives discovered:**

| Path | Description |
|------|-------------|
| User → Azure AD Role (Global Admin, etc.) | Direct or transitive role assignment |
| Group → Azure AD Role | Group assigned to privileged role via Azure AD PIM |
| Service Principal → Azure RBAC Role | SPN with Owner/Contributor on subscription/resource |
| App → Privileged Role | Application with delegated app permissions |
| Privileged Role → All Users | Global Admin can reset any user password, modify any tenant setting |
| Hybrid — User → AD Group → Azure Role | Synced on-prem group with cloud role |

## Manual

### Authentication

```bash
# Interactive device code auth (recommended)
azurehound

# Client credentials (app registration)
azurehound -c -tid <tenant-id> -cid <client-id> -cs <client-secret>

# Azure CLI token (reuse cached az login)
azurehound -a

# Username/password (MFA will block)
azurehound -u user@domain.com -p 'Password!'
```

### Collection

```bash
# Collect everything
azurehound -o output.json

# Specify tenant explicitly
azurehound -o output.json -tid <tenant-id>

# Collect only Azure AD (no RBAC)
azurehound --azure-ad-only -o output.json

# Collect only Azure RBAC
azurehound --azure-rbac-only -o output.json

# Verbose output
azurehound --verbose -o output.json
```

### BloodHound Ingestion

```bash
# Upload to BloodHound CE via web UI
# Data Import -> Upload File -> select output.json

# Or via API (BloodHound CE)
curl -X POST \
  -H "Authorization: Bearer <TOKEN>" \
  -F "file=@output.json" \
  https://bloodhound-ce/api/bloodhound/data
```

### Connect to Azure for collection

```powershell
# Preregister an Azure AD app with required permissions
# Required API permissions: Directory.Read.All, User.Read.All, Group.Read.All
# RoleManagement.Read.Directory, Application.Read.All, Device.Read.All

# Or use Global Administrator account (device code flow)
```

## Build

```bash
git clone https://github.com/BloodHoundAD/AzureHound.git
cd AzureHound

# Install Go dependencies
go mod download

# Build
go build -o azurehound .
```

## Install

### Linux / macOS

```bash
# Download from GitHub releases
wget https://github.com/BloodHoundAD/AzureHound/releases/latest/download/azurehound-linux-amd64.zip
unzip azurehound-linux-amd64.zip
chmod +x azurehound

# macOS
wget https://github.com/BloodHoundAD/AzureHound/releases/latest/download/azurehound-darwin-amd64.zip
```

### Windows

```powershell
Invoke-WebRequest -Uri https://github.com/BloodHoundAD/AzureHound/releases/latest/download/azurehound-windows-amd64.zip -OutFile azurehound.zip
Expand-Archive -Path azurehound.zip -DestinationPath .
.\azurehound.exe
```

## Package

| Manager | Command |
|---------|---------|
| GitHub Releases | https://github.com/BloodHoundAD/AzureHound/releases |
| Go install | `go install github.com/BloodHoundAD/AzureHound@latest` |

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/BloodHoundAD/AzureHound |
| Releases | https://github.com/BloodHoundAD/AzureHound/releases |
| BloodHound docs | https://bloodhound.readthedocs.io/ |
| Azure AD attack paths | https://posts.specterops.io/azure-hound-80618088d95b |
| Cloud attack paths | https://posts.specterops.io/ |
