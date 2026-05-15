# Master Documentation: Assignments 1, 2, and 3

## Executive Summary

The three assignments form a certificate management platform. Assignment 1 is the initial Rust certificate metadata backend. Assignment 2 is the production architecture for secure certificate issuance and inventory in Kubernetes. Assignment 3 is a Next.js operational frontend that integrates with the Rust backend over TLS-aware server-side calls.

The core engineering principle across all three is separation of concerns: HTTP transport, business policy, persistence, security, observability, deployment, and frontend rendering are deliberately separated so the system can grow without becoming a monolith.

## Assignment 1: Rust Certificate Backend

Assignment 1 uses Rust, Axum, Tokio, SQLx, and PostgreSQL. Its modules are intentionally conventional:

- `main.rs` starts the runtime and server.
- `config.rs` loads environment configuration.
- `routes` wires Axum routes and middleware.
- `handlers` translate HTTP requests and responses.
- `services` apply validation and business rules.
- `repositories` isolate SQL.
- `models` define domain and DTO structs.
- `errors.rs` maps application failures to HTTP responses.

This structure is used because it keeps code reviewable. A senior engineer can inspect API behavior in handlers, business rules in services, and SQL safety in repositories without reading a single monolithic file.

Tokio is appropriate because the service is I/O-bound. SQLx is appropriate because it gives async pooling and parameterized queries. PostgreSQL is appropriate because certificate inventory queries are relational and index-friendly.

## Assignment 2: Secure Microservice Architecture

Assignment 2 expands the same backend idea into a production service. The service accepts a CSR, validates policy, signs through a CA abstraction, stores inventory metadata, and writes audit evidence.

### Why Rust, Axum, and Tokio

Rust provides memory safety and high performance without a garbage collector. Axum is chosen because it is explicit and lightweight. Tokio is chosen because HTTP, PostgreSQL, Vault/KMS calls, and telemetry export are all async I/O.

### Why PostgreSQL and SQLx

PostgreSQL is used because certificate metadata has strong relational access patterns: tenant, issuer, subject, SAN, expiration, status, serial number, and audit timeline. SQLx is used because it supports typed async access, connection pooling, and parameterized SQL.

### Why the Modules Exist

The Assignment 2 `domain` module contains certificate issuance concepts and validation. The `http` module contains Axum routes, DTOs, timeouts, and request limits. The `infra` module contains PostgreSQL and signing adapters. The `observability` module initializes structured tracing. The `security` module is reserved for request identity and authorization helpers.

This is a ports-and-adapters style. The domain depends on traits, not concrete infrastructure. That allows dummy signing to be replaced by Vault PKI or AWS Private CA while keeping handlers and policy logic stable.

### Certificate Flow

CSR submission enters the Axum handler. The handler builds a domain command using request data and authenticated identity. The domain validates subject, SANs, and TTL. The signing adapter signs through a CA backend. The repository writes inventory and audit data. The response returns certificate id, serial number, PEM, and expiration.

### Alternatives Avoided

Storing CA private keys in the API pod was avoided because compromise of the API would become compromise of the CA. A monolithic handler was avoided because it makes security review difficult. Browser-direct backend calls were avoided in Assignment 3 because they expose internal topology and complicate credential handling.

## Database Design

Assignment 2 uses separate tables for tenants, issuance requests, certificates, and audit events. This design preserves failed request history, supports inventory operations, and keeps security audit evidence queryable.

Indexes are chosen around real workflows: expiration for renewal, tenant/status/expiration for dashboards, subject for operator search, GIN on SAN entries for DNS identity search, and tenant/time indexes for audit timelines.

## Kubernetes Design

The Kubernetes package includes Deployment, Service, ConfigMap, Secret, HPA, Ingress, NetworkPolicy, ServiceMonitor, and Istio mTLS policy.

Deployment controls rollouts and pod replicas. Service provides stable discovery. ConfigMap separates environment-specific non-secret values. Secret is a placeholder for externally managed sensitive values. HPA scales stateless API pods. Ingress handles north-south TLS. NetworkPolicy restricts lateral movement. ServiceMonitor connects Prometheus. Istio policy enforces mTLS and workload authorization.

## TLS and mTLS

TLS protects traffic entering the platform. mTLS protects service-to-service traffic inside Kubernetes. Istio or Linkerd is preferred for mTLS because certificate rotation, peer identity, and policy enforcement belong to the platform layer.

Application authorization still matters. mTLS authenticates workload identity; it does not decide whether an actor may issue `CN=payments.example.com` for a given tenant.

## Observability

`tracing` is used for structured application logs. OpenTelemetry is used for distributed traces. Prometheus is used for metrics. Grafana is used for dashboards.

Important signals include HTTP latency, 4xx/5xx rates, issuance success/failure, database pool pressure, signing backend latency, expiring certificates, and audit write failures.

Audit logs are treated as security evidence. They should be immutable or append-only, retained according to policy, and exported to a SIEM.

## Assignment 3: Next.js Frontend

Assignment 3 uses Next.js App Router, TypeScript strict mode, TailwindCSS, SSR, and SWR. It implements `/inventory`, dashboard cards, details pages, filtering, pagination, loading states, error states, and reusable UI components.

SSR is chosen because the page can fetch securely from the backend server-side, render quickly, and avoid exposing backend URLs or credentials in browser JavaScript. SWR is chosen for lightweight revalidation after SSR. React Query would be reasonable for heavier mutation workflows, but SWR is a better fit for a read-mostly inventory dashboard.

The frontend uses a backend-for-frontend route at `/inventory/api`. The browser calls Next.js; Next.js calls Assignment 1. This flow keeps TLS handling and future service credentials on the server.

## Assignment 3 Integration Contract

Assignment 1 exposes `GET /certificates`, so the frontend can render an inventory table without manual certificate id configuration. The current frontend applies filtering and pagination in the Next.js server layer because the Assignment 1 backend is deliberately minimal. In production, the backend should expose indexed query parameters so PostgreSQL performs filtering and pagination close to the data.

## Docker and Container Security

Both backend and frontend Dockerfiles use multi-stage builds. Runtime images should run as non-root, avoid build tools, and be scanned in CI. Production should add SBOMs, image signing, pinned base images, and vulnerability gates.

## Performance and Scalability

Rust API pods scale horizontally, but PostgreSQL must be protected with bounded connection pools and indexed queries. Certificate issuance depends on signing backend latency, so timeouts and clear error metrics are important. Audit tables may need time partitioning at scale.

Next.js can also scale horizontally. Because certificate inventory can be sensitive, caching should be explicit and conservative.

## Interview Talking Points

You can explain the system as a secure certificate platform with a Rust control-plane API, PostgreSQL-backed inventory, CA signing abstraction, Kubernetes deployment, service mesh mTLS, structured observability, audit logging, and a server-rendered Next.js operations UI.

The strongest senior-level points are:

- Private signing keys stay outside the API service.
- mTLS and authorization solve different problems.
- Audit logs are separate from diagnostic logs.
- PostgreSQL indexing follows operational workflows.
- SSR protects backend topology and credentials.
- Async Rust improves I/O concurrency, but bounded pools and timeouts are the real production controls.
- Kubernetes hardening includes non-root pods, read-only filesystems, network policy, probes, HPA, and externalized secrets.

## Where to Read Next

- `assignment1/README.md`
- `assignment1/DEEP_DIVE.md`
- `assignment-2/README.md`
- `assignment-2/docs/ARCHITECTURE.md`
- `assignment-2/docs/SECURITY.md`
- `assignment-2/docs/OBSERVABILITY.md`
- `assignment-2/docs/DEEP_DIVE.md`
- `assignment-3/README.md`
- `assignment-3/docs/ARCHITECTURE.md`
- `assignment-3/docs/TLS_AND_MTLS.md`
- `assignment-3/docs/DEEP_DIVE.md`
- `INTERVIEW_PREP_AI_USAGE.md`
