# Assignment 2 Deep Dive

This document explains Assignment 2 in interview depth: architecture, each file's purpose, each module's role, and why the design choices were made.

## Scope and Intent

Assignment 2 is a secure microservice system-design package for certificate issuance and inventory. It contains:

- Rust reference implementation skeleton.
- PostgreSQL schema and indexes.
- Kubernetes manifests.
- mTLS/service mesh policy examples.
- Observability, security, and hardening documentation.

The code is intentionally representative rather than feature-complete production business logic.

## Rust Service: Module Map

### `src/main.rs`

Role:

- Wires config, tracing, DB, repository, signer, and HTTP router.

Why:

- Central composition root keeps dependency wiring explicit.
- Traits are injected to preserve replaceability (dummy signer -> Vault/KMS signer).

### `src/config/mod.rs`

Role:

- Reads env into `Settings`.

Why:

- Decouples runtime configuration from code.
- Keeps image immutable across environments.

### `src/domain/model.rs`

Role:

- Defines issuance command and issued certificate models.

Why:

- Domain models capture business intent independently of Axum/SQL.

### `src/domain/ports.rs`

Role:

- Defines interfaces (`CertificateRepository`, `CertificateAuthority`).

Why:

- Ports-and-adapters architecture.
- Enables testing and backend replacement with minimal churn.

### `src/domain/issuance.rs`

Role:

- Validates issuance policy.
- Orchestrates sign + persist flow.

Why:

- Business decisions belong in domain/service layer, not handlers.
- Keeps control-plane logic testable without HTTP or DB wiring.

### `src/http/router.rs`

Role:

- Declares routes.
- Applies body-size limits, request timeout, and tracing middleware.

Why:

- Security and resilience controls should be centralized and consistent.

### `src/http/handlers.rs`

Role:

- Translates request JSON into domain command.
- Calls domain issuer service and maps response.

Why:

- Thin handler approach keeps transport concerns separate.

### `src/infra/postgres.rs`

Role:

- Creates SQLx pool.
- Persists audit evidence (and transaction scaffolding for inventory writes).

Why:

- Persistence details are isolated from business logic.
- Pool limits protect DB under horizontal scaling.

### `src/infra/signing.rs`

Role:

- Dummy certificate authority implementation.

Why:

- Allows architecture validation before integrating real CA backends.
- Preserves same interface that Vault/AWS KMS adapter would implement.

### `src/observability/tracing.rs`

Role:

- Initializes structured JSON logs.

Why:

- JSON logs are machine-queryable for production observability tools.

### `src/security/mod.rs`

Role:

- Placeholder boundary for authn/authz helpers.

Why:

- Keeps security cross-cutting logic grouped rather than scattered.

## Schema and Data Model

File: `db/schema.sql`

Core tables:

- `tenants`: tenant boundary.
- `certificate_issuance_requests`: lifecycle of requested issuance.
- `certificates`: final issued inventory records.
- `audit_events`: immutable-style security evidence.

Why separate request and certificate tables:

- Failed/rejected requests are operationally important and should not vanish.
- Maintains clean lifecycle tracking and auditability.

Index strategy:

- `(tenant_id, status, not_after)` for renewal and dashboards.
- `subject` for operator search.
- GIN on SAN array for identity lookup.
- tenant/time indexes for audit and request timelines.

## API Contract

File: `api/openapi.yaml`

Key endpoints:

- `GET /health`
- `GET /ready`
- `POST /v1/certificates:issue`
- `GET /v1/certificates`
- `GET /v1/certificates/{certificate_id}`

Why this API shape:

- Health/readiness separation supports proper K8s probes.
- Inventory endpoints match operator workflows.
- Issue endpoint is explicit control-plane action.

## Kubernetes and Platform Files

### `k8s/namespace.yaml`

- Isolates workload and enables namespace-level policy.

### `k8s/configmap.yaml`

- Non-secret runtime config (`BIND_ADDR`, log level, OTEL endpoint).

### `k8s/secret.yaml`

- Secret placeholders (DB URL, Vault token).
- Production should replace with external secret manager.

### `k8s/deployment.yaml`

- Desired pod state, probes, resources, security context, annotations.

### `k8s/service.yaml`

- Stable cluster DNS endpoint for API pods.

### `k8s/hpa.yaml`

- Horizontal scaling based on resource metrics.

### `k8s/ingress.yaml`

- External routing and TLS termination.

### `k8s/network-policy.yaml`

- Restricts allowed ingress/egress paths.

### `k8s/istio-mtls.yaml`

- Enforces STRICT mTLS.
- Adds workload-level authorization policy example.

### `k8s/observability/servicemonitor.yaml`

- Prometheus scrape integration for metrics.

## Security Reasoning

- Private signing keys never reside in API pods.
- mTLS provides workload identity and encrypted transport.
- Authorization still needed because mTLS is not user intent.
- Request body limits reduce abuse/memory pressure.
- Parameterized SQL prevents injection.
- Non-root runtime and read-only filesystem reduce blast radius.

## Async and Concurrency Reasoning

- Workload is I/O-bound (DB + signer + telemetry).
- Tokio async runtime maximizes concurrent throughput per pod.
- Concurrency is intentionally bounded via DB pool and HTTP controls.

## Known Gap in Skeleton

The current Postgres adapter writes the tenant placeholder, issuance request, certificate inventory row, and audit event in one transaction. That transactional boundary is important because a certificate issuance workflow should not return success if the inventory or audit evidence failed to persist. In a production service, the same structure would use SQLx query macros checked against migrations in CI and a real fingerprint derived from the issued PEM.

## Interview Narrative

Use this short narrative:

"I designed Assignment 2 as a secure certificate control-plane service. I separated transport, domain policy, and infrastructure through ports-and-adapters so dummy issuance could be replaced with Vault or AWS KMS signing. I modeled issuance requests, inventory, and audit evidence separately in Postgres, then provided Kubernetes manifests for deployment, scaling, ingress, mesh mTLS, and observability. I intentionally documented production controls like strict mTLS, non-root containers, request limits, and audit event durability."
