# Certificate Service

A production-style Rust microservice for managing certificate metadata and parsing PEM certificates.

## Architecture Overview

This service is built using a clean layered architecture:

- `src/main.rs` initializes the runtime, tracing, configuration, and the router.
- `src/config.rs` loads environment configuration using `dotenvy`.
- `src/db/mod.rs` manages PostgreSQL connection pooling with `sqlx`.
- `src/routes/mod.rs` defines the HTTP routes and middleware.
- `src/handlers` contains transport layer handlers for REST endpoints.
- `src/services` contains business logic and input validation.
- `src/repositories` contains persistence logic and SQLx queries.
- `src/models` contains request/response DTOs and domain data structures.
- `src/errors.rs` defines application error handling and HTTP translation.

## Features

- POST `/certificates`
- GET `/certificates/:id`
- POST `/parse-certificate`
- GET `/health`
- Structured async logging with `tracing`
- Compile-time checked SQL queries using `sqlx`
- PostgreSQL `TEXT[]` support for SAN entries
- Embedded startup migrations using SQLx
- Secure multi-stage Docker build
- Docker Compose deployment with PostgreSQL

## Getting Started

### Prerequisites

- Rust stable toolchain
- Docker & Docker Compose
- PostgreSQL (or use Docker Compose)

### Local Setup

1. Copy `.env.example` to `.env`
2. Update `DATABASE_URL` if needed
3. Run migrations manually if you want to prepare the database before starting the app. The service also applies embedded migrations on startup.

For Bash/macOS/Linux:

```bash
cargo install sqlx-cli --no-default-features --features postgres
DATABASE_URL="postgres://postgres:3010@localhost:5433/certificates" sqlx migrate run
```

For Windows CMD:

```cmd
cargo install sqlx-cli --no-default-features --features postgres
set "DATABASE_URL=postgres://postgres:3010@localhost:5433/certificates" && sqlx migrate run
```

For Windows PowerShell:

```powershell
cargo install sqlx-cli --no-default-features --features postgres
$env:DATABASE_URL = 'postgres://postgres:3010@localhost:5433/certificates'
sqlx migrate run
```

4. Run the service locally:

```bash
cargo run --release
```

### Optional SQLx compile-time validation

When you want compile-time SQL validation, enable the `sqlx-check` feature and provide either a live `DATABASE_URL` or a prepared `.sqlx` cache:

```bash
DATABASE_URL="postgres://postgres:3010@localhost:5433/certificates" cargo check --features sqlx-check
```

To prepare an offline cache for future builds:

```bash
cargo sqlx prepare -- --features sqlx-check
```

### Docker

Start the app and database with Docker Compose:

```bash
docker compose up --build
```

The service will listen on `http://127.0.0.1:8080`.

## API Examples

### Create a certificate

PowerShell example with an expiration date 30 days from now:

```powershell
$body = @{
  subject = "CN=example.com"
  issuer = "CN=Example CA"
  expiration = (Get-Date).ToUniversalTime().AddDays(30).ToString("o")
  san_entries = @("example.com", "www.example.com")
} | ConvertTo-Json

Invoke-RestMethod http://127.0.0.1:8080/certificates -Method Post -ContentType "application/json" -Body $body
```

### Get certificate metadata

```bash
curl http://127.0.0.1:8080/certificates/<certificate_id>
```

### Parse a PEM certificate

```bash
curl -X POST http://127.0.0.1:8080/parse-certificate \
  -H "Content-Type: application/json" \
  -d '{"certificate_pem": "-----BEGIN CERTIFICATE-----..."}'
```

## Environment Variables

- `DATABASE_URL`: PostgreSQL connection string
- `SERVER_ADDR`: address for the HTTP server
- `RUST_LOG`: tracing log filter

## Security Considerations

- Uses a non-root user in the runtime container
- Employs a minimal distroless runtime image
- Avoids `unwrap()` and uses structured error handling
- Uses parameterized SQL through `sqlx` to prevent injection

## Scalability Considerations

- Connection pooling with `sqlx::PgPool`
- Router and service layers separate transport, business logic, and persistence
- Trace logging middleware can be extended with distributed tracing
- Docker Compose service separation enables horizontal scaling behind a load balancer

