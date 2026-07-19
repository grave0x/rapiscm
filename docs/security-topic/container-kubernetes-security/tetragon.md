# Tetragon — eBPF-Based Security Observability & Enforcement

eBPF runtime security engine with enforcement capabilities (SIGKILL, syscall rejection). Native Cilium integration. Detects and blocks threats at kernel level.

## How It Works

Tetragon uses eBPF programs attached to kernel tracepoints, kprobes, and LSM hooks. It observes process execution, file access, network connections, and syscalls. Unlike Falco (detect-only), Tetragon can enforce — killing processes or blocking syscalls inline. Policies are defined via YAML `TracingPolicy` CRDs.

**Key features:**
- Process execution monitoring (execve family)
- File access monitoring (open, read, write) with LSM enforcement
- Network monitoring (connect, accept, send, recv)
- Capability and privilege tracking
- Kernel-level enforcement via LSM (SIGKILL, syscall rejection)
- Native Cilium integration for pod identity

## Manual

```bash
# Run with direct eBPF
tetragon --bpf-lib /var/lib/tetragon

# Export events to JSON
tetragon --export-filename /var/log/tetragon.log

# Observe process execution events
tetra getevents -o json | jq '.process_exec'
```

### Kubernetes (Helm)

```bash
# Install Tetragon with Cilium integration
helm repo add cilium https://helm.cilium.io
helm install tetragon cilium/tetragon -n kube-system
```

### TracingPolicy Example

```yaml
apiVersion: cilium.io/v1alpha1
kind: TracingPolicy
metadata:
  name: "block-write-etc"
spec:
  kprobes:
  - call: "security_file_permission"
    syscall: false
    args:
    - index: 0
      type: file
    selectors:
    - matchArgs:
      - index: 0
        operator: "Prefix"
        values:
        - "/etc/"
      matchActions:
      - action: SIGKILL
```

## Build

```bash
git clone https://github.com/cilium/tetragon.git
cd tetragon
make build
# Binaries in ./build/
```

## Install

```bash
# Binary download
curl -L https://github.com/cilium/tetragon/releases/latest/download/tetragon-linux-amd64.tar.gz | tar xz

# Docker
docker run -ti --rm \
  --pid=host \
  --cgroupns=host \
  --privileged \
  -v /sys/kernel/btf:/sys/kernel/btf:ro \
  quay.io/cilium/tetragon:v1.0

# Helm (K8s)
helm repo add cilium https://helm.cilium.io
helm install tetragon cilium/tetragon -n kube-system
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/cilium/tetragon |
| Docs | https://tetragon.cilium.io/docs/ |
| TracingPolicy examples | https://github.com/cilium/tetragon/tree/main/examples/tracingpolicy |
| Cilium integration | https://cilium.io/ |
