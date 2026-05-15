# Assignment 1 Deep Dive

This document explains Assignment 1 file-by-file, function-by-function, and design-decision-by-design-decision. It is written for interview preparation.

## Purpose

Assignment 1 is a Rust certificate metadata microservice. It provides:

- `POST /certificates` to create metadata records.
- `GET /certificates/:id` to fetch one certificate record.
- `POST /parse-certificate` to parse PEM certificate contents.
- `GET /health` for operational health checks.

The scope is intentionally narrow so architecture quality is visible: clear layering, typed DTOs, safe persistence, and structured errors.

## Architectural Pattern

The service follows a layered architecture:

- Transport layer: Axum routes + handlers.
- Service layer: validation + business flow.
- Repository layer: SQLx persistence.
- Model layer: domain structs + DTOs.
- Infrastructure layer: config + DB pool.

Why this is used:

- Easier testing and code ownership.
- Cleaner security reviews because responsibilities are not mixed.
- Future-friendly: transport or storage can change without rewriting everything.

## File-By-File Guide

### `src/main.rs`

Responsibilities:

- Bootstraps runtime (`tokio`).
- Loads env config.
- Initializes tracing/logging.
- Creates PostgreSQL pool.
- Applies migrations.
- Builds router and starts HTTP server.

Important function:

- `main()`: orchestration entrypoint.

Why this shape:

- Startup concerns are centralized.
- Runtime failures are visible early (migrations, DB connectivity).

### `src/config.rs`

Responsibilities:

- Reads env variables into `AppConfig`.
- Applies safe defaults for local runs.

Important function:

- `AppConfig::load()`.

Why this shape:

- Keeps configuration parsing out of business code.
- Makes behavior predictable across local/dev/prod.

### `src/db/mod.rs`

Responsibilities:

- Defines `DbPool` alias.
- Creates SQLx connection pool with timeouts and max connections.

Important function:

- `create_pool(&AppConfig)`.

Why this shape:

- Pooling prevents per-request connection creation overhead.
- Connection limits avoid overloading PostgreSQL.

### `src/routes/mod.rs`

Responsibilities:

- Declares API endpoints.
- Applies shared middleware (`TraceLayer`).
- Injects application state (`DbPool`) into handlers.

Why this shape:

- Routing stays declarative.
- Shared middleware is applied once and consistently.

### `src/handlers/certificate.rs`

Responsibilities:

- Parses HTTP inputs (path/body/state).
- Calls service methods.
- Returns JSON responses.

Important functions:

- `create_certificate()`
- `get_certificate()`
- `parse_certificate()`

Why this shape:

- Handlers remain thin and transport-focused.
- Business logic is delegated to service layer.

### `src/handlers/health.rs`

Responsibilities:

- Returns health status for liveness checks.

Why this shape:

- Provides a simple endpoint for container probes and monitoring checks.

### `src/services/certificate_service.rs`

Responsibilities:

- Validates inputs.
- Builds domain model for persistence.
- Orchestrates repository calls.
- Parses PEM certificates with `x509-parser`.

Important functions:

- `create_certificate(pool, request)`
- `get_certificate(pool, certificate_id)`
- `parse_certificate(request)`
- `extract_san_entries(cert)`

Why this shape:

- Service layer is where business policy belongs.
- Parsing and domain-specific decisions stay out of transport and repository code.

### `src/repositories/certificate_repository.rs`

Responsibilities:

- Executes SQL statements for insert and lookup.
- Maps DB errors into app errors.

Important functions:

- `insert_certificate(pool, certificate)`
- `find_certificate_by_id(pool, certificate_id)`

Why two SQLx modes:

- `cfg(sqlx_checked)` path for compile-time checked SQL in strict pipelines.
- fallback path for flexible local iteration.

### `src/models/certificate.rs`

Responsibilities:

- Domain persistence model (`Certificate`) with SQLx row mapping.

Why this shape:

- Keeps domain fields explicit and typed.
- Reused across service and repository boundaries.

### `src/models/dto.rs`

Responsibilities:

- Request/response contracts.
- Input validation rules for create endpoint.

Important methods:

- `CreateCertificateRequest::validate()`

Why this shape:

- API contracts remain explicit.
- Validation rules are centralized and testable.

### `src/errors.rs`

Responsibilities:

- Defines application error taxonomy.
- Maps errors to HTTP responses.

Why this shape:

- Consistent client behavior.
- Safer than ad-hoc `unwrap`/string errors.

### `migrations/20260514_create_certificates_table.sql`

Responsibilities:

- Creates `certificates` table and supporting indexes.

Design:

- `UUID` primary key for distributed-safe identifiers.
- `TIMESTAMPTZ` for clear time semantics.
- `TEXT[]` for SAN entries with non-empty constraint.
- indexes on `expiration` and `subject` for common lookup patterns.

### `tests/integration.rs`

Responsibilities:

- End-to-end behavior checks using the service API boundaries.

Why this matters:

- Validates integration across handler/service/repository stack.

### `Dockerfile`

Responsibilities:

- Multi-stage build for smaller runtime image.
- Non-root execution for baseline container hardening.

### `docker-compose.yml`

Responsibilities:

- Local orchestration for service + PostgreSQL.

## API Flow Walkthrough

Create certificate flow:

1. `POST /certificates` enters handler.
2. Handler deserializes JSON DTO.
3. Service validates business rules.
4. Service constructs `Certificate`.
5. Repository inserts row.
6. Service returns id.
7. Handler returns `201`-style JSON payload.

Parse certificate flow:

1. `POST /parse-certificate` enters handler.
2. Service parses PEM block.
3. Service decodes X.509 DER.
4. Service extracts subject, issuer, expiration, SAN.
5. Handler returns parsed metadata.

## Why These Libraries

- `axum`: explicit and lightweight HTTP framework.
- `tokio`: async runtime optimized for I/O-heavy backends.
- `sqlx`: safe async Postgres + optional compile-time SQL checks.
- `tracing`: structured logging and request diagnostics.
- `x509-parser`: reliable X.509 parsing.

## Interview Talking Points

- Clear layering avoids monolithic endpoint implementations.
- Validation happens before persistence to keep data clean.
- Connection pooling and timeouts are first-class operational concerns.
- Structured errors and tracing improve debuggability and reliability.
- SQL remains parameterized and typed to reduce security risk.
