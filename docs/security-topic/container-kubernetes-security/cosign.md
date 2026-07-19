# Cosign — Container Image Signing & Verification

Container signing and verification tool. Part of the Sigstore project. Keyless signing via OIDC identity, transparency log verification, and Kubernetes admission enforcement.

## How It Works

Cosign signs container images and blobs by generating a digital signature stored in an OCI registry (as a separate tag or referrers API artifact). The signature binds the image digest to an identity (email, OIDC issuer) via the Fulcio root CA, logged in the Rekor transparency log for auditability.

**Signing modes:**

| Mode | Description |
|------|-------------|
| **Key-pair** | Generate local keys (`cosign generate-key-pair`), sign with private key, verify with public |
| **Keyless** | OIDC-based identity (GitHub, Google, Microsoft). Fulcio issues short-lived cert bound to identity. Rekor logs the signature. |
| **Blob** | Sign arbitrary files (binaries, SBOMs, policies) with the same keyless flow |
| **Attestation** | Sign in-toto attestations for SLSA provenance, vulnerability scan results |

## Manual

```bash
# Sign container image (keyless)
cosign sign ghcr.io/myorg/myimage:latest

# Sign with local key pair
cosign generate-key-pair
cosign sign --key cosign.key ghcr.io/myorg/myimage:latest

# Verify
cosign verify ghcr.io/myorg/myimage:latest

# Verify with specific identity
cosign verify --certificate-identity-email dev@example.com \
  --certificate-oidc-issuer https://accounts.google.com \
  ghcr.io/myorg/myimage:latest

# Sign a blob
cosign sign-blob --output-certificate cert.pem --output-signature sig.bin binary.bin

# Verify blob
cosign verify-blob --certificate cert.pem --signature sig.bin binary.bin

# Generate SBOM attestation
cosign attest --predicate sbom.cdx.json --type cyclonedx ghcr.io/myorg/myimage:latest
```

### Kubernetes Admission (with Kyverno or OPA)

```yaml
apiVersion: cosigned.sigstore.dev/v1alpha1
kind: ClusterImagePolicy
metadata:
  name: enforce-signed
spec:
  images:
    - glob: "ghcr.io/myorg/*"
  authorities:
    - keyless:
        identities:
          - issuer: "https://accounts.google.com"
            subject: "dev@example.com"
```

## Build

```bash
git clone https://github.com/sigstore/cosign.git
cd cosign
make cosign
# Binary in ./cosign
```

## Install

```bash
# Linux (script)
curl -O -L "https://github.com/sigstore/cosign/releases/latest/download/cosign-linux-amd64"
sudo mv cosign-linux-amd64 /usr/local/bin/cosign
sudo chmod +x /usr/local/bin/cosign

# macOS
brew install cosign

# Docker
docker pull ghcr.io/sigstore/cosign/cosign:latest

# Go install
go install github.com/sigstore/cosign/v2/cmd/cosign@latest
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.sigstore.dev/ |
| GitHub | https://github.com/sigstore/cosign |
| Docs | https://docs.sigstore.dev/cosign/overview/ |
| Sigstore Quickstart | https://docs.sigstore.dev/quickstart/ |
| Rekor | https://github.com/sigstore/rekor |
| Fulcio | https://github.com/sigstore/fulcio |
