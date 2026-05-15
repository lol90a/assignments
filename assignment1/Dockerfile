# Build stage contains the Rust toolchain and native dependencies only.
FROM rust:1-slim AS builder

WORKDIR /usr/src/certificate_service

RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock build.rs ./
COPY src ./src
COPY migrations ./migrations

# Fetch dependencies before compiling so Docker can cache crates between source edits.
RUN cargo fetch
RUN cargo build --release

# Runtime stage is distroless to keep the shipped image small and minimal.
FROM gcr.io/distroless/cc

WORKDIR /app
COPY --from=builder /usr/src/certificate_service/target/release/certificate_service /app/certificate_service

USER 1000:1000

EXPOSE 8080
ENTRYPOINT ["/app/certificate_service"]
