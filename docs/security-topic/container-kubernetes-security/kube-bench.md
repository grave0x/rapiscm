# kube-bench — CIS Kubernetes Benchmark Auditor

CIS Benchmark compliance checker for Kubernetes. Runs checks against control plane, worker nodes, etcd, RBAC, and policies.

## How It Works

kube-bench runs a series of automated checks defined in CIS Benchmark YAML configs. Each check tests a specific recommendation (e.g., "--anonymous-auth=false"), compares against the running cluster configuration, and reports pass/fail/warn.

**Check categories:**

| Section | Target |
|---------|--------|
| 1 | Control Plane Node Configuration |
| 2 | etcd Configuration |
| 3 | Control Plane Configuration Files |
| 4 | Worker Nodes |
| 5 | Policies (RBAC, ServiceAccounts, Network, Secrets) |
| 6 | Managed Kubernetes (GKE, EKS, AKS — auto-detection) |

## Manual

```bash
# Run all checks
kube-bench run

# Run specific section
kube-bench run --targets master,node

# Run with specific config version
kube-bench run --version 1.24

# Output as JSON
kube-bench run --json --output kube-bench-results.json

# Run as Job on cluster
kubectl apply -f job.yaml  # see https://github.com/aquasecurity/kube-bench/main/deploy/

# Use custom config
kube-bench run --config-dir /path/to/cfg --config cfg/config.yaml
```

### Job YAML

```yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: kube-bench
spec:
  template:
    metadata:
      labels:
        app: kube-bench
    spec:
      hostPID: true
      containers:
        - name: kube-bench
          image: ghcr.io/aquasecurity/kube-bench:latest
          volumeMounts:
            - name: var-lib-etcd
              mountPath: /var/lib/etcd
            - name: var-lib-kubelet
              mountPath: /var/lib/kubelet
      restartPolicy: Never
```

## Build

```bash
git clone https://github.com/aquasecurity/kube-bench.git
cd kube-bench
make build
# Binary in ./kube-bench
```

## Install

```bash
# macOS
brew install kube-bench

# Docker
docker pull ghcr.io/aquasecurity/kube-bench:latest
docker run --rm -v /etc/kubernetes:/etc/kubernetes:ro \
  ghcr.io/aquasecurity/kube-bench:latest

# Linux binary
curl -L https://github.com/aquasecurity/kube-bench/releases/download/v0.9.3/kube-bench_0.9.3_linux_amd64.tar.gz
tar xvf kube-bench_0.9.3_linux_amd64.tar.gz
sudo install kube-bench /usr/local/bin/
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/aquasecurity/kube-bench |
| CIS Benchmarks | https://www.cisecurity.org/benchmark/kubernetes |
| Deploy as Job | https://github.com/aquasecurity/kube-bench/blob/main/job.yaml |
| Supported versions | https://github.com/aquasecurity/kube-bench#cis-benchmark-versions |
