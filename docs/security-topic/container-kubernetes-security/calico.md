# Calico — Kubernetes Networking & Network Security

Container networking and network policy engine. Provides CNI, network policy enforcement, and workload segmentation for Kubernetes, VMs, and bare metal.

## How It Works

Calico implements a flat IP fabric using BGP for routing (no overlay required). Network policies map to iptables/nftables rules on each node. Supports eBPF data plane for better performance.

**Key capabilities:**

| Feature | Description |
|---------|-------------|
| **Networking** | CNI with BGP routing, overlay (VXLAN/IPIP) option, IP pools, NAT |
| **Network Policy** | Kubernetes NetworkPolicy + Calico extended policies. Namespace and global scope. |
| **Service Graph** | Visual policy editor and traffic flow dashboard |
| **eBPF** | High-performance eBPF data plane as kube-proxy replacement |
| **Microsegmentation** | Host endpoint policies for VMs, bare metal, and workloads outside K8s |
| **WireGuard** | Pod-to-pod encryption at runtime |

## Manual

```bash
# Install on cluster
kubectl create -f https://raw.githubusercontent.com/projectcalico/calico/main/manifests/calico.yaml

# Or via Helm
helm repo add projectcalico https://docs.tigera.io/calico/charts
helm install calico projectcalico/tigera-operator --namespace tigera-operator --create-namespace

# CLI (calicoctl)
calicoctl get nodes
calicoctl get ippools
calicoctl get networkpolicies -n default

# Global network policy (Calico extended)
cat <<EOF | kubectl apply -f -
apiVersion: crd.projectcalico.org/v1
kind: GlobalNetworkPolicy
metadata:
  name: deny-all-egress
spec:
  selector: all()
  egress:
    - action: Deny
EOF

# Enable eBPF
calicoctl patch felixconfiguration default --patch \
  '{"spec": {"bpfEnabled": true}}'

# View flow logs
calicoctl get flows -n default
```

### Policy Tiers

```yaml
apiVersion: crd.projectcalico.org/v1
kind: Tier
metadata:
  name: security
spec:
  order: 100
---
apiVersion: crd.projectcalico.org/v1
kind: NetworkPolicy
metadata:
  name: security.deny-all
spec:
  tier: security
  selector: all()
  ingress:
    - action: Deny
```

## Build

```bash
# Calico is usually deployed via manifests, not built from source
git clone https://github.com/projectcalico/calico.git
cd calico
make build
```

## Install

```bash
# Kubernetes manifest (quick start)
kubectl create -f https://raw.githubusercontent.com/projectcalico/calico/main/manifests/calico.yaml

# Helm (production)
helm repo add projectcalico https://docs.tigera.io/calico/charts
helm install calico projectcalico/tigera-operator --namespace tigera-operator

# calicoctl CLI
curl -L https://github.com/projectcalico/calico/releases/latest/download/calicoctl-linux-amd64
sudo mv calicoctl-linux-amd64 /usr/local/bin/calicoctl
sudo chmod +x /usr/local/bin/calicoctl
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.tigera.io/project-calico/ |
| GitHub | https://github.com/projectcalico/calico |
| Docs | https://docs.tigera.io/calico/latest/ |
| Calico Cloud | https://www.tigera.io/tigera-products/calico-cloud/ |
| Policy Editor | https://www.tigera.io/security-policy-builder/ |
