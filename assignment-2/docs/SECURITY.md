# Security, TLS, and mTLS

## API Security

External callers should authenticate through OIDC/JWT at the ingress or application layer. The Rust service should still validate important claims itself because ingress-only enforcement can be bypassed by misconfiguration or internal callers. Authorization checks should bind the actor to a tenant and allowed certificate policy.

## TLS for North-South Traffic

The public endpoint terminates TLS at the ingress gateway using a certificate issued by a trusted CA. TLS protects users from passive network interception and active tampering between browser, frontend, gateway, and backend edge.

## mTLS for East-West Traffic

Inside the cluster, Istio or Linkerd issues workload identities and transparently upgrades service-to-service connections to mTLS. This means traffic between the Next.js server, Rust backend, OpenTelemetry collector, and other services is encrypted and mutually authenticated.

mTLS is handled by sidecars or ambient mesh because hand-writing certificate rotation and peer verification in every service is error-prone. The application still enforces authorization because mTLS answers "which workload is calling," not "is this actor allowed to issue this certificate."

## Secret Management

Kubernetes Secrets in the manifests are placeholders. Production should use External Secrets Operator, Vault Agent Injector, Secrets Store CSI Driver, or cloud-native secret projection. Database passwords, signing backend tokens, and telemetry credentials should never be baked into images or committed to Git.

## Signing Key Protection

The certificate authority private key should live in Vault PKI, AWS Private CA, or an HSM/KMS-backed service. The Rust service receives only signed certificates, not signing keys. This is the main blast-radius control for a certificate authority workflow.

## Audit Logging

Audit events are persisted to PostgreSQL as append-only rows. They include actor id, tenant id, action, target resource, decision, source IP/workload, request id, and structured details. Diagnostic logs are useful for debugging; audit logs are security evidence and must be retained, protected from casual deletion, and exported to SIEM in production.

## Hardening Checklist

- Run as non-root with a read-only root filesystem.
- Drop Linux capabilities.
- Use minimal images and pin dependency versions.
- Add request body size limits for CSRs.
- Rate limit issuance endpoints.
- Validate subject and SAN policy before signing.
- Use parameterized SQL through SQLx.
- Rotate database credentials and signing backend credentials.
- Enforce mesh peer authentication in STRICT mode.
- Use network policies so only expected workloads can reach the API and database.
