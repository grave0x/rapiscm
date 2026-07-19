# Pacu — AWS Exploitation Framework

## How It Works

Pacu is an open-source AWS security testing framework by Rhino Security Labs for assessing IAM privileges, enumerating resources, and demonstrating cloud-specific attack paths.

**Key architecture:**

- **Module system** — Python modules organized by MITRE ATT&CK tactics for cloud:
  - `recon/` — IAM role enumeration, user/group/role/policy discovery
  - `enum/` — S3 bucket listing, Lambda function enumeration, EC2/ELB data gathering
  - `iam__*` — privilege escalation, backdoor creation, policy manipulation
  - `ebs__*` — EBS snapshot exfiltration, volume enumeration
  - `ec2__*` — SSRF exploitation metadata, instance backdooring
  - `lambda__*` — Lambda function backdooring via code injection
  - `rds__*` — RDS snapshot/public access enumeration
  - `cloudtrail__*` — logging evasion via trail disabling/deletion
- **Session management** — save/load AWS sessions with role assumptions, MFA sessions
- **Dependency injection** — boto3 clients per session, auto-configure from assumed role
- **Reporting** — HTML + JSON export of findings per module
- **SQLite database** — store enumerated data for cross-module correlation

**Key attack capabilities:**
- Privilege escalation via IAM policy analysis (identify and exploit dangerous permission combos)
- S3 bucket enumeration and data exfiltration
- Lambda persistence via function code injection
- EBS snapshot sharing to attacker-controlled account
- CloudTrail/GuardDuty detection evasion
- EC2 instance user-data extraction (often contains secrets)

## Manual

### Launch

```bash
pacu
# > help — list commands
# > ls — list modules
```

### Key Commands (Interactive)

```bash
# Configure session
> set_keys            # set AWS access keys
> import_keys         # import from env, file, or AWS CLI profile
> set_regions all     # target all regions
> swap_region us-east-1

# Reconnaissance
> run iam__enum_users_roles_policies
> run iam__enum_permissions
> run ec2__enum
> run s3__enum

# Privilege escalation
> run iam__bruteforce_permissions
> run iam__privesc_scan
> run iam__backdoor_assume_role

# Data exfiltration
> run ebs__enum_volumes_snapshots
> run ebs__share_snapshot
> run rds__enum

# Persistence
> run lambda__backdoor_new_roles
> run lambda__backdoor_existing_functions

# Detection evasion
> run cloudtrail__disabler
> run guardduty__whitelist_ip
```

### Synthetic Run

```bash
# Automate end-to-end assessment
# Non-interactive mode
pacu --session mysession --run-synth

# Output report
pacu --session mysession --report
```

## Build

```bash
git clone https://github.com/RhinoSecurityLabs/pacu.git
cd pacu
pip install -r requirements.txt
```

## Install

```bash
pip install pacu

# Or from source
git clone https://github.com/RhinoSecurityLabs/pacu.git
cd pacu
bash install.sh
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/RhinoSecurityLabs/pacu |
| Wiki | https://github.com/RhinoSecurityLabs/pacu/wiki |
| Module list | https://github.com/RhinoSecurityLabs/pacu/wiki/Module-Details |
| Rhino Security Labs | https://rhinosecuritylabs.com/ |
