# Secure Certificate Issuance Microservice Architecture

## Goals

The service is designed to issue X.509 certificates, store searchable inventory metadata, expose secure APIs to trusted clients, and run as a hardened cloud-native workload. Dummy issuance is acceptable for the assessment, but the architecture deliberately separates certificate policy, signing, storage, and transport concerns so the dummy signer can be replaced by Vault PKI or an AWS KMS-backed CA without rewriting handlers.

## Main Components

### Rust API Service

The API is implemented with Axum on Tokio. Axum is intentionally lightweight: it provides routing, extractors, middleware integration, and good Tower compatibility without hiding HTTP behavior behind a large framework. Tokio is the async runtime because database I/O, network calls to Vault/KMS, and telemetry exports are all I/O-bound. Async lets each pod handle many concurrent requests with a small thread pool instead of blocking one OS thread per request.

### PostgreSQL

PostgreSQL stores certificate metadata, issuance requests, and audit events. The certificate body may be stored for inventory convenience, but private keys are never stored by this service. Postgres is preferred over a document database because inventory queries typically filter by expiration, issuer, status, SAN, serial number, tenant, and creation time. Those access patterns map cleanly to B-tree and GIN indexes.

### Signing Backend

The service depends on a `CertificateAuthority` trait. In production, the trait would call Vault PKI, AWS Private CA, or an internal CA. The private key stays outside the API pod. This avoids making API compromise equivalent to CA key compromise.

### Kubernetes

Kubernetes runs the service as a Deployment with readiness and liveness probes, ConfigMaps for non-secret configuration, Secrets for bootstrap values, an HPA for horizontal scaling, and an Ingress/Gateway for north-south traffic.

### Service Mesh

Istio or Linkerd provides workload identity and mTLS for east-west traffic. The mesh is used for transport security and service identity, not as a replacement for API authorization. This separation matters because mTLS proves which workload called the service; authorization still decides whether that workload can issue a certificate for a subject or tenant.

### Observability

The service emits structured logs with `tracing`, metrics for Prometheus, and distributed traces through OpenTelemetry. Audit logs are separate from diagnostic logs because audit records are evidence and need stronger integrity, retention, and query semantics.

## Certificate Issuance Flow

1. A trusted client submits a CSR with requested metadata and tenant context.
2. The API authenticates the caller and authorizes the requested subject/SANs against policy.
3. The CSR is parsed and validated. Validation happens before persistence so malformed input does not pollute inventory tables.
4. The service writes an issuance request row with status `pending`.
5. The CA adapter signs the CSR using Vault PKI or AWS KMS concepts.
6. The service stores certificate metadata, PEM material, serial number, validity bounds, SANs, and status in PostgreSQL in the same transaction that finalizes the issuance request.
7. An immutable audit event records actor, action, target, decision, request id, and source workload.
8. The response returns the issued certificate and inventory id.

## Why Clean Module Boundaries

- `http` owns transport concerns such as JSON extraction, status codes, and middleware.
- `domain` owns certificate concepts, policy validation, and business errors.
- `infra` owns PostgreSQL, Vault/KMS adapters, and other external systems.
- `observability` owns telemetry wiring so handlers do not need to know exporter details.
- `security` owns authentication, authorization, and request identity extraction.

This boundary keeps the API framework from leaking into core certificate logic. It also makes the signing backend replaceable and testable.

## Alternatives Avoided

- A monolithic handler file was avoided because it couples HTTP, SQL, signing, and audit behavior in one place, making security review harder.
- Synchronous database calls were avoided because blocking calls reduce pod concurrency and make latency under load less predictable.
- Storing CA private keys in the API container was avoided because it dramatically increases blast radius.
- Relying only on mesh mTLS was avoided because authenticated workload identity is not the same as user or tenant authorization.
