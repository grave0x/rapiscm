# Falco — Cloud-Native Runtime Security

eBPF-based runtime security engine for containers, Kubernetes, and hosts. CNCF graduated project. Detects anomalous syscall activity with 100+ default rules mapped to MITRE ATT&CK.

## How It Works

Falco hooks into syscalls via kernel module, eBPF probe, or modern BPF. It evaluates events against a rule engine (Lua-based) that matches syscall fields, container context, and K8s metadata. Outputs go to stdout, gRPC, or sidecar files. Alerts can trigger Falcosidekick for further integration.

**Detection categories:**
- Container escape (mount namespace, --privileged)
- Reverse shell,可疑 network connections
- Sensitive file access (/etc/shadow, kubelet, SSH keys)
- Privilege escalation (setuid binaries, new capabilities)
- K8s-specific (configmap creation, privileged pod, cluster-admin binding)

## Manual

```bash
# Run with eBPF probe (recommended)
falco --bpf

# Run with kernel module
falco

# Custom rule file
falco --rules-file /path/to/rules.yaml

# Output to JSON
falco --json-output

# Log to file
falco --log-output --log-file /var/log/falco.log
```

### Helm (Kubernetes)

```bash
helm repo add falcosecurity https://falcosecurity.github.io/charts
helm install falco falcosecurity/falco \
  --set driver.kind=ebpf \
  --set falcosidekick.enabled=true
```

### Custom Rule Example

```yaml
- rule: Write Below Binary Dir
  desc: writing to binary directory
  condition: >
    open_write and bin_dir and not package_mgmt
  output: "file below binary dir (user=%user.name command=%proc.cmdline file=%fd.name)"
  priority: WARNING
  tags: [mitre_persistence]
```

## Build

```bash
git clone https://github.com/falcosecurity/falco.git
cd falco
mkdir build && cd build
cmake -DUSE_BUNDLED_DEPS=ON ..
make -j$(nproc)
# Binaries in build/userspace/falco/
```

## Install

```bash
# Linux (script)
curl -s https://falco.org/script/install.sh | bash

# DEB/RPM
# Add Falcosecurity APT/YUM repos then:
apt install falco   # Debian/Ubuntu
yum install falco   # RHEL/CentOS

# Docker
docker run -i -t \
  --privileged \
  -v /var/run/docker.sock:/var/run/docker.sock \
  falcosecurity/falco

# Helm (K8s)
helm repo add falcosecurity https://falcosecurity.github.io/charts
helm install falco falcosecurity/falco
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://falco.org/ |
| GitHub | https://github.com/falcosecurity/falco |
| Rules | https://github.com/falcosecurity/rules |
| Falcosidekick | https://github.com/falcosecurity/falcosidekick |
| Docs | https://falco.org/docs/ |
