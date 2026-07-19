# Container & Kubernetes Security

Tools and techniques for securing containerized workloads and Kubernetes clusters across the full lifecycle — image scanning, runtime protection, admission control, posture management, network policy, and supply chain integrity.

## Sub-Topics

| Topic | Description |
|-------|-------------|
| Image Vulnerability Scanning | Static analysis of container layers against CVE databases (NVD, GHSA, OSV) |
| Runtime Security | eBPF/kprobe syscall monitoring for container escape, reverse shells, privilege escalation |
| Admission Control | Intercept CREATE/UPDATE API requests; validate manifests against policies |
| Pod Security Standards | Privileged / Baseline / Restricted profiles via PSA |
| CIS Benchmarks | kube-bench (K8s), Docker Bench (host) |
| Secrets Management | External Secrets Operator, Vault, sealed secrets |
| SBOM & Supply Chain | CycloneDX/SPDX generation, Sigstore/Cosign signing, SLSA provenance |
| Network Policies | Default-deny, L3-L7 microsegmentation with Calico, Cilium |
| Cluster Posture | Periodic NSA/CISA + MITRE ATT&CK auditing |
| Image Signing | Keyless OIDC signing with Cosign, verification at admission |

## Methods

- **Image scanning** — scan layers against NVD/GHSA/OSV; gate builds on CRITICAL/HIGH with `--ignore-unfixed`
- **Runtime detection** — eBPF hooks on syscalls; detect escape, reverse shell, sensitive file access; alert or enforce (SIGKILL)
- **Admission control** — validate manifests against Rego/YAML policies; enforce registry allowlisting, no-root, no-privileged
- **Supply chain** — sign images keyless (Cosign + Fulcio); store in Rekor; verify at admission via Kyverno/Sigstore Policy Controller
- **Posture scanning** — periodic cluster audit against NSA/CISA + MITRE ATT&CK for Containers
- **Network microsegmentation** — default-deny ingress/egress; L7-aware with Cilium eBPF; pod-to-pod encryption

## Tool Comparison

| Tool | Category | Deployment | Key Strength |
|------|----------|------------|--------------|
| **Trivy** | Scanner | CLI, Operator, CI | All-in-one: CVEs, secrets, IaC, K8s |
| **Falco** | Runtime | DaemonSet, Host | eBPF syscall rules, 100+ defaults |
| **Tetragon** | Runtime | DaemonSet | eBPF enforcement (SIGKILL, block), Cilium native |
| **OPA Gatekeeper** | Admission | Admission Webhook | Rego DSL, external data, any K8s resource |
| **Kyverno** | Admission | Admission Webhook | YAML policies (no DSL), validate/mutate/generate |
| **Cosign** | Supply Chain | CLI, Admission | Keyless OIDC signing, Rekor integration |
| **Kubescape** | Posture | CLI, Operator | NSA/CISA + MITRE mapping, auto netpols |
| **kube-bench** | Posture | Pod, Host | CIS K8s Benchmark checks |
| **Checkov** | IaC | CLI, CI | Multi-platform IaC scanning |
| **Clair** | Scanner | Service | Container layer CVE scanning |
| **Cilium** | Network | CNI | eBPF L3-L7, encryption, Tetragon companion |
| **Calico** | Network | CNI | GlobalNetworkPolicy, egress controls |
