# ModSecurity + OWASP CRS — Web Application Firewall

Open-source WAF engine (ModSecurity) + rule set (OWASP Core Rule Set). Industry-standard for web application protection. Identifies and blocks OWASP Top 10 attacks.

## How It Works

ModSecurity is a WAF module for Apache, Nginx, and IIS. Inspects HTTP traffic against rules. Three-phase detection: request headers, request body, response body.

**CRS rule categories:**

| Category | ID Range | Description |
|----------|----------|-------------|
| REQUEST-901-INITIALIZATION | 901xxx | Engine initialization and config |
| REQUEST-903.9001-DRUPAL | 903xxx | Drupal-specific rules |
| REQUEST-903.9002-WORDPRESS | 903xxx | WordPress-specific rules |
| REQUEST-905-COMMON-EXCEPTIONS | 905xxx | False positive tuning exceptions |
| REQUEST-910-IP-REPUTATION | 910xxx | IP reputation blocklists |
| REQUEST-911-METHOD-ENFORCEMENT | 911xxx | HTTP method restrictions |
| REQUEST-912-DOS-PROTECTION | 912xxx | DoS detection |
| REQUEST-913-SCANNER-DETECTION | 913xxx | Scanner/user-agent detection |
| REQUEST-920-PROTOCOL-ENFORCEMENT | 920xxx | Protocol/header validation |
| REQUEST-921-PROTOCOL-ATTACK | 921xxx | Protocol attack detection |
| REQUEST-930-APPLICATION-ATTACK-LFI | 930xxx | Local file inclusion |
| REQUEST-931-APPLICATION-ATTACK-RFI | 931xxx | Remote file inclusion |
| REQUEST-932-APPLICATION-ATTACK-RCE | 932xxx | Remote code execution |
| REQUEST-933-APPLICATION-ATTACK-PHP | 933xxx | PHP injection |
| REQUEST-934-APPLICATION-ATTACK-GENERIC | 934xxx | Generic injection |
| REQUEST-941-APPLICATION-ATTACK-XSS | 941xxx | Cross-site scripting |
| REQUEST-942-APPLICATION-ATTACK-SQLI | 942xxx | SQL injection |
| REQUEST-943-APPLICATION-ATTACK-SESSION-FIXATION | 943xxx | Session fixation |
| REQUEST-944-APPLICATION-ATTACK-JAVA | 944xxx | Java injection |
| REQUEST-949-BLOCKING-EVALUATION | 949xxx | Blocking decision |
| RESPONSE-950-DATA-LEAKAGES | 950xxx | Data leak detection |
| RESPONSE-959-BLOCKING-EVALUATION | 959xxx | Response blocking |
| RESPONSE-980-CORRELATION | 980xxx | Alert correlation |

**Paranoia levels (PL):** 1 (low false positives, good baseline), 2 (stricter, some false positives), 3 (heavy, tune first), 4 (maximum, highest false positive rate).

## Manual

```bash
# Apache: enable module and load CRS
sudo a2enmod headers
sudo a2enmod security2
sudo systemctl restart apache2
# CRS at: /etc/apache2/conf-enabled/owasp-crs.conf

# Nginx: load ModSecurity module
# In nginx.conf:
# load_module modules/ngx_http_modsecurity_module.so;
# In server block:
# modsecurity on;
# modsecurity_rules_file /etc/nginx/modsec/main.conf;

# Test rule match
curl -X GET "http://target.com/?q=<script>alert(1)</script>"
# Expected: 403 Forbidden + audit log entry

# Check audit log
cat /var/log/modsec_audit.log
# Tail in real time
tail -f /var/log/modsec_audit.log | grep -E "CRS|Inbound|Outbound"

# CRS tuning: exclude specific rules
SecRuleRemoveById 942100 942110  # Remove SQLi rules for specific URL
SecRuleUpdateTargetById 942100 "!REQUEST_COOKIES"  # Exclude cookies
```

## Build

```bash
# ModSecurity v3 (libmodsecurity)
git clone https://github.com/owasp-modsecurity/ModSecurity.git
cd ModSecurity
git submodule init && git submodule update
./build.sh
./configure && make && sudo make install

# With Nginx connector
git clone https://github.com/owasp-modsecurity/ModSecurity-nginx.git
# Build as dynamic module for nginx
```

## Install

```bash
# Debian/Ubuntu (Apache)
sudo apt install libapache2-mod-security2
sudo cp /etc/modsecurity/owasp-crs/crs-setup.conf.example \
  /etc/modsecurity/owasp-crs/crs-setup.conf

# Debian/Ubuntu (Nginx)
sudo apt install libnginx-mod-modsecurity

# Docker with ModSecurity + CRS
docker pull owasp/modsecurity-crs:nginx
docker run -e PARANOIA=2 -p 80:80 owasp/modsecurity-crs:nginx

# Manual CRS download
git clone https://github.com/coreruleset/coreruleset.git /etc/modsecurity/coreruleset/
```

## Package

| Manager | Command |
|---------|---------|
| apt | `sudo apt install libapache2-mod-security2` |
| Docker | `docker pull owasp/modsecurity-crs:nginx` |
| CRS source | `git clone https://github.com/coreruleset/coreruleset.git` |

## Links

| Resource | URL |
|----------|-----|
| ModSecurity GitHub | https://github.com/owasp-modsecurity/ModSecurity |
| OWASP CRS | https://coreruleset.org/ |
| CRS GitHub | https://github.com/coreruleset/coreruleset |
| CRS docs | https://coreruleset.org/docs/ |
| Docker | https://github.com/coreruleset/modsecurity-crs-docker |
| Troubleshooting | https://coreruleset.org/docs/troubleshooting/ |
