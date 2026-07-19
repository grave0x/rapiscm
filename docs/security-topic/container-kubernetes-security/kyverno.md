# Kyverno — Kubernetes-Native Policy Engine

Kubernetes admission control and policy engine using YAML policies (no DSL). CNCF graduated. Supports validate, mutate, generate, and cleanup policies with built-in image verification.

## How It Works

Kyverno runs as a dynamic admission webhook. Policies are written as Kubernetes Custom Resources in pure YAML. It matches resources by kind, labels, annotations, and names. Built-in variables (`request.userInfo`, `object.metadata`, etc.) provide rich context without needing a separate policy language.

**Key features:**
- **Validate** — enforce Pod Security Standards, required labels, forbidden registries
- **Mutate** — inject sidecars, add labels/annotations, default securityContext
- **Generate** — create related resources (NetworkPolicy, ConfigMap) when parent created
- **Cleanup** — delete expired or orphaned resources
- **Image verification** — verify Cosign signatures at admission time
- **Policy reports** — background scanning for existing resources

## Manual

```bash
# Install Kyverno
kubectl create -f https://raw.githubusercontent.com/kyverno/kyverno/main/config/install.yaml

# Check status
kubectl get pods -n kyverno
```

### Validate Policy Example

```yaml
apiVersion: kyverno.io/v1
kind: ClusterPolicy
metadata:
  name: require-run-as-nonroot
spec:
  validationFailureAction: Enforce
  rules:
  - name: check-run-as-nonroot
    match:
      any:
      - resources:
          kinds:
          - Pod
    validate:
      message: "Running as root is not allowed"
      pattern:
        spec:
          securityContext:
            runAsNonRoot: true
```

### Mutate + Image Verify Example

```yaml
apiVersion: kyverno.io/v1
kind: ClusterPolicy
metadata:
  name: add-sidecar
spec:
  rules:
  - name: inject-envoy
    match:
      any:
      - resources:
          kinds:
          - Pod
    mutate:
      patchStrategicMerge:
        spec:
          containers:
          - name: envoy-sidecar
            image: envoyproxy/envoy:v1.28
---
apiVersion: kyverno.io/v1
kind: ClusterPolicy
metadata:
  name: verify-image
spec:
  rules:
  - name: verify-cosign
    match:
      any:
      - resources:
          kinds:
          - Pod
    verifyImages:
    - image: "registry.io/*"
      key: |-
        -----BEGIN PUBLIC KEY-----
        ...
        -----END PUBLIC KEY-----
```

## Build

```bash
git clone https://github.com/kyverno/kyverno.git
cd kyverno
make build
```

## Install

```bash
# Helm
helm repo add kyverno https://kyverno.github.io/kyverno/
helm install kyverno kyverno/kyverno -n kyverno --create-namespace

# Manifest
kubectl create -f https://raw.githubusercontent.com/kyverno/kyverno/main/config/install.yaml

# High availability
helm install kyverno kyverno/kyverno -n kyverno --create-namespace --set replicaCount=3
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://kyverno.io/ |
| GitHub | https://github.com/kyverno/kyverno |
| Policy library | https://kyverno.io/policies/ |
| Docs | https://kyverno.io/docs/ |
| Playground | https://playground.kyverno.io/ |
