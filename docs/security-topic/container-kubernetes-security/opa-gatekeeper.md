# OPA Gatekeeper — Kubernetes Admission Controller

Policy-based admission controller using the Rego policy language. CNCF graduated. Intercepts Kubernetes API CREATE/UPDATE/DELETE requests and validates against configurable policies.

## How It Works

Gatekeeper integrates the Open Policy Agent (OPA) as a Kubernetes admission webhook. Policies are defined as `ConstraintTemplate` CRDs (Rego templates) and instantiated via `Constraint` resources. It can validate, mutate, and audit resources.

**Key concepts:**
- **ConstraintTemplate** — Rego policy template with parameters
- **Constraint** — instantiation of a template with specific parameters (e.g., "require label 'owner'")
- **Audit** — periodic evaluation of all existing resources (not just admission)
- **Data** — external data injection for context-aware policies

## Manual

```bash
# Install Gatekeeper
kubectl apply -f https://raw.githubusercontent.com/open-policy-agent/gatekeeper/master/deploy/gatekeeper.yaml

# Check status
kubectl get pods -n gatekeeper-system
```

### ConstraintTemplate + Constraint Example

```yaml
# ConstraintTemplate
apiVersion: templates.gatekeeper.sh/v1
kind: ConstraintTemplate
metadata:
  name: k8srequiredlabels
spec:
  crd:
    spec:
      names:
        kind: K8sRequiredLabels
      validation:
        openAPIV3Schema:
          type: object
          properties:
            labels:
              type: array
              items:
                type: string
  targets:
    - target: admission.k8s.gatekeeper.sh
      rego: |
        package k8srequiredlabels
        violation[{"msg": msg}] {
          provided := {label | input.review.object.metadata.labels[label]}
          required := {label | label := input.parameters.labels[_]}
          missing := required - provided
          count(missing) > 0
          msg := sprintf("missing labels: %v", [missing])
        }
---
# Constraint
apiVersion: constraints.gatekeeper.sh/v1beta1
kind: K8sRequiredLabels
metadata:
  name: require-owner-label
spec:
  match:
    kinds:
      - apiGroups: [""]
        kinds: ["Pod"]
  parameters:
    labels: ["owner"]
```

### Mutating Policy

```yaml
apiVersion: mutations.gatekeeper.sh/v1
kind: Assign
metadata:
  name: add-sidecar-annotation
spec:
  applyTo:
    - groups: [""]
      versions: ["v1"]
      kinds: ["Pod"]
  match:
    source: "All"
  location: "metadata.annotations.sidecar-injected"
  parameters:
    assign:
      value: "true"
```

## Build

```bash
git clone https://github.com/open-policy-agent/gatekeeper.git
cd gatekeeper
make build
# Binary in ./bin/
```

## Install

```bash
# Production deploy (Helm)
helm repo add gatekeeper https://open-policy-agent.github.io/gatekeeper/charts
helm install gatekeeper/gatekeeper --name-template=gatekeeper --namespace gatekeeper-system --create-namespace

# Manifest deploy
kubectl apply -f https://raw.githubusercontent.com/open-policy-agent/gatekeeper/master/deploy/gatekeeper.yaml
```

## Package

- Helm chart: `oci://openpolicyagent.github.io/gatekeeper/charts/gatekeeper`
- Container image: `openpolicyagent/gatekeeper:v3.15.0`

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/open-policy-agent/gatekeeper |
| Docs | https://open-policy-agent.github.io/gatekeeper/website/docs/ |
| Policy library | https://github.com/open-policy-agent/gatekeeper-library |
| OPA (Rego) | https://www.openpolicyagent.org/docs/latest/policy-language/ |
