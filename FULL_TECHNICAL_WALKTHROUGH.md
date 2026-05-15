# Full Technical Walkthrough

This walkthrough explains Assignments 1, 2, and 3 as one platform: a Rust certificate backend, a secure microservice architecture for certificate issuance and inventory, and a Next.js operational frontend.

## 1. System Purpose

The platform manages certificate metadata and certificate issuance workflows. Assignment 1 implements the initial Rust backend. Assignment 2 expands that backend into a production-grade Kubernetes architecture with mTLS, observability, audit logging, and secure signing concepts. Assignment 3 adds a professional frontend for inventory operations.

## 2. Assignment 1 Backend

Assignment 1 is a Rust microservice using Axum, Tokio, SQLx, and PostgreSQL. It exposes health, certificate creation, certificate lookup, and PEM parsing endpoints. The code is split into configuration, routing, handlers, services, repositories, models, database setup, and errors.

The important architectural choice is layered separation. Handlers translate HTTP into application calls. Services hold validation and business workflow. Repositories own SQL. Models and DTOs define stable data contracts. This keeps HTTP concerns away from persistence and makes future API changes safer.

Tokio is used because the service spends most of its time waiting on network and database I/O. Async execution lets a small number of runtime worker threads handle many concurrent requests. This is better than blocking per request for a microservice expected to sit behind a load balancer.

SQLx is chosen because it provides async PostgreSQL access, connection pooling, typed row mapping, and optional compile-time SQL validation. Parameterized SQL avoids injection risks. PostgreSQL is chosen because certificate inventory is naturally relational and benefits from indexes on expiration and subject.

## 3. Assignment 2 Architecture

Assignment 2 models the production version of the certificate platform. The Rust service accepts CSRs, validates policy, delegates signing, stores metadata, and writes audit events.

The architecture separates the certificate authority behind a trait. Dummy issuance is acceptable for the assessment, but production signing should use Vault PKI, AWS Private CA, KMS/HSM-backed workflows, or another dedicated CA service. The API container should never hold CA private keys.

The domain layer contains issuance policy and certificate workflow. The HTTP layer contains Axum routes, JSON DTOs, timeouts, tracing middleware, and request body limits. The infrastructure layer contains PostgreSQL and signing adapters. Observability and security have dedicated modules so cross-cutting concerns do not become scattered handler code.

## 4. Library Choices

Rust is used for memory safety, predictable performance, and low runtime overhead. Axum is used because it is small, Tower-compatible, and explicit about request extraction and response construction. Tokio is used because the workload is I/O-heavy. SQLx is used for safe async PostgreSQL access. `tracing` provides structured logs and spans. OpenTelemetry provides vendor-neutral distributed traces. Prometheus and Grafana provide metrics and operational dashboards.

Next.js is used for Assignment 3 because the frontend needs SSR, route-level data loading, a backend-for-frontend pattern, and modern TypeScript ergonomics. TailwindCSS is used for consistent enterprise UI styling without introducing a large component framework. SWR is used for lightweight client revalidation after SSR.

## 5. Database Design

Assignment 1 has a compact `certificates` table with certificate id, subject, issuer, expiration, and SAN entries. Assignment 2 expands the schema into tenants, issuance requests, certificates, and audit events.

Issuance requests are separate from certificates because failed or rejected requests matter for investigation. Audit events are separate because audit records are evidence, not ordinary diagnostic logs. SAN entries use a PostgreSQL array with a GIN index because operators commonly search by DNS name or identity. Expiration indexes support renewal workflows and dashboard queries.

## 6. API Design

Assignment 1 exposes a minimal API. Assignment 2 documents a production API under `/v1`, including `POST /v1/certificates:issue`, `GET /v1/certificates`, and `GET /v1/certificates/{id}`. The list endpoint belongs in the backend because pagination, filtering, and expiration queries should use database indexes rather than pushing large datasets to the browser.

Assignment 3 integrates with the existing Assignment 1 API by calling `GET /certificates` for inventory and `GET /certificates/:id` for details. Filtering and pagination are performed in the Next.js server layer for this assessment, but production inventory should move those operations into indexed PostgreSQL queries behind backend query parameters.

## 7. Security Reasoning

Input validation happens before persistence or signing. Certificate subject, SAN entries, TTL, and CSR structure must be checked against policy. The backend should authenticate callers through OIDC/JWT and authorize by tenant and certificate policy.

TLS protects north-south traffic. mTLS protects east-west service traffic in Kubernetes. The mesh gives workload identity and automatic certificate rotation, but application authorization remains necessary. mTLS proves which workload called the service; it does not prove that a human actor may issue a certificate for a tenant.

Secrets are never hard-coded. Kubernetes Secrets in the manifests are placeholders only. Production should use Vault, External Secrets Operator, Secrets Store CSI Driver, or cloud-native secret managers. Signing keys should remain in Vault, AWS Private CA, KMS, or HSM-backed systems.

## 8. TLS and mTLS

For local development, a self-signed certificate can be used behind a local reverse proxy. Assignment 3 includes documentation for generating local certificates and explicitly flags the self-signed bypass as local-only.

For Kubernetes, Istio or Linkerd should run mTLS in strict mode. The Assignment 2 manifests include `PeerAuthentication` and `AuthorizationPolicy` examples. Service mesh mTLS is preferred over application-managed mTLS because rotation, peer identity, and policy enforcement are operational platform concerns.

## 9. Observability

Diagnostic logs use `tracing` because structured fields are queryable and can be correlated with spans. Metrics should expose request rate, latency, error count, certificate issuance outcomes, signing backend latency, database pool saturation, and audit write failures. OpenTelemetry traces connect HTTP requests, database work, signing calls, and audit writes.

Audit logging is intentionally separate. Audit records must include actor, tenant, action, target, decision, request id, source workload, source IP, and structured details. Audit write failures deserve alerts because they create compliance and forensic risk.

## 10. Kubernetes Reasoning

The Assignment 2 Kubernetes manifests include Namespace, Deployment, Service, ConfigMap, Secret, HPA, Ingress, NetworkPolicy, ServiceMonitor, and Istio security policy.

The Deployment uses multiple replicas, resource requests and limits, probes, non-root execution, dropped capabilities, and a read-only root filesystem. The Service provides stable internal discovery. HPA scales stateless API pods. Ingress terminates external TLS. NetworkPolicy reduces lateral movement. ServiceMonitor enables Prometheus scraping.

## 11. Docker and Container Security

Rust builds use multi-stage containers so the runtime image does not contain compilers or build caches. The runtime image uses a non-root user. Next.js also uses a build stage and a runtime stage. Production images should be scanned, pinned, signed, and accompanied by SBOMs.

## 12. Assignment 3 Frontend

The frontend is an App Router Next.js application with strict TypeScript. `/inventory` is server-rendered. The detail route `/inventory/[id]` is also server-rendered. `InventoryTable` is a client component because it uses SWR, interactive filtering links, loading states, and error states.

The browser talks to Next.js. Next.js talks to the Rust backend. This backend-for-frontend pattern keeps internal backend URLs and credentials off the browser, simplifies CORS, and provides one place to handle TLS and service credentials.

SSR was chosen over pure CSR because certificate inventory is operational data and should render reliably before client JavaScript finishes. SWR is layered on top because operators still expect live-ish refresh behavior without full page reloads.

## 13. Async and Concurrency

The Rust backend uses async for network and database operations. The most important concurrency control is not spawning unlimited tasks; it is bounding resources. SQLx pools bound database concurrency. HTTP body limits bound memory exposure. Timeouts bound dependency waits. Kubernetes HPA adds pod-level concurrency only when capacity is needed.

The frontend uses server-side fetches during SSR and SWR revalidation in the browser. The Next.js server acts as the concurrency boundary for backend calls.

## 14. Performance and Scalability

The Rust service is lightweight and can scale horizontally. PostgreSQL is the stateful bottleneck, so indexes, connection pooling, and query design matter more than raw Rust CPU speed. Inventory list queries should be paginated and backed by indexes. Audit tables may eventually need partitioning by time.

Next.js can scale horizontally as a stateless frontend. Caching should be conservative because certificate state can be security-sensitive, but short server-side revalidation windows may be acceptable for read-only inventory screens.

## 15. Production Readiness

Before production, add authentication, authorization policies, a real CA adapter, migration jobs, CI dependency scanning, image signing, CSP headers, rate limiting, request id propagation, alert rules, SIEM export, backup/restore testing, and disaster recovery runbooks.

The system is intentionally designed so these additions fit into existing boundaries instead of requiring a rewrite.

## 16. Study Pack

For line-by-line interview preparation, read:

- `assignment1/DEEP_DIVE.md`
- `assignment-2/docs/DEEP_DIVE.md`
- `assignment-3/docs/DEEP_DIVE.md`
- `INTERVIEW_PREP_AI_USAGE.md`
