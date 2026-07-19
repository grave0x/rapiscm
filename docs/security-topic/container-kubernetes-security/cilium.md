# Cilium — eBPF-Based Networking, Observability & Security

eBPF-powered CNI for Kubernetes. Provides networking, network policy, load balancing, and observability — all without sidecars.

## How It Works

Cilium leverages Linux eBPF to inject network, security, and observability programs directly into the kernel. For Kubernetes, it replaces kube-proxy and implements CNI with native eBPF data paths.

**Key capabilities:**

| Feature | Description |
|---------|-------------|
| **Network Policy** | L3/L4 and L7 policies. Identity-based (no pod IPs). Ingress/Egress. HTTP, gRPC, Kafka aware. |
| **Hubble** | Real-time service map, flow logs, metrics, monitoring UI. Built-in observability layer. |
| **Cluster Mesh** | Multi-cluster networking and policy enforcement across clusters. |
| **Service Mesh** | Envoy-based L7 with eBPF acceleration. Sidecar-free option. |
| **Load Balancing** | Maglev consistent hashing, XDP acceleration, DSR. |
| **Encryption** | WireGuard or IPsec for pod-to-pod encryption. |
| **Tetragon** | Runtime security observability via eBPF (separate component). |

## Manual

```bash
# Install on cluster (via Helm)
helm repo add cilium https://helm.cilium.io/
helm install cilium cilium/cilium --namespace kube-system

# Check status
cilium status
cilium clusterhealth

# Network policy example (L3/L4)
cat <<EOF | kubectl apply -f -
apiVersion: cilium.io/v2
kind: CiliumNetworkPolicy
metadata:
  name: allow-frontend
spec:
  endpointSelector:
    matchLabels:
      app: backend
  ingress:
    - fromEndpoints:
        - matchLabels:
            app: frontend
      toPorts:
        - ports:
            - port: "8080"
              protocol: TCP
EOF

# Hubble UI
cilium hubble enable --ui
cilium hubble port-forward&
# Open http://localhost:12000

# Flow monitoring
hubble observe
hubble observe --from-pod default/frontend --to-pod default/backend
hubble observe --verdict DROPPED
```

### Connectivity Test

```bash
# Run connectivity test suite
cilium connectivity test
```

## Build

```bash
git clone https://github.com/cilium/cilium.git
cd cilium
make build
```

## Install

```bash
# Helm (recommended)
helm repo add cilium https://helm.cilium.io/
helm install cilium cilium/cilium --namespace kube-system

# CLI tools
# Linux
curl -L --remote-name-all https://github.com/cilium/cilium-cli/releases/latest/download/cilium-linux-amd64.tar.gz
sudo tar xzvfC cilium-linux-amd64.tar.gz /usr/local/bin
rm cilium-linux-amd64.tar.gz

# macOS
brew install cilium-cli

# Hubble CLI
curl -L --remote-name-all https://github.com/cilium/hubble/releases/latest/download/hubble-linux-amd64.tar.gz
sudo tar xzvfC hubble-linux-amd64.tar.gz /usr/local/bin
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://cilium.io/ |
| GitHub | https://github.com/cilium/cilium |
| Docs | https://docs.cilium.io/ |
| Hubble | https://github.com/cilium/hubble |
| Cilium Policy Editor | https://editor.cilium.io/ |
| Slack | https://cilium.slack.com/ |
