# Multi-stage build keeps compilers and cargo cache out of the runtime image.
FROM rust:1.78-bookworm AS builder
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app
COPY --from=builder /app/target/release/assignment-2-certificate-platform /app/certificate-platform
USER nonroot:nonroot
EXPOSE 8080
ENTRYPOINT ["/app/certificate-platform"]
