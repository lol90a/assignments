# TLS and mTLS Setup

## Local TLS

Assignment 1 can be placed behind a local TLS proxy such as Caddy, Nginx, or Traefik, or the Rust service can be extended with `rustls`. For local self-signed development:

```bash
openssl req -x509 -newkey rsa:4096 -nodes \
  -keyout certs/local.key \
  -out certs/local.crt \
  -days 30 \
  -subj "/CN=127.0.0.1"
```

Point a local reverse proxy at Assignment 1 and set:

```text
ASSIGNMENT1_API_BASE_URL=https://127.0.0.1:8443
ALLOW_SELF_SIGNED_CERTS=true
```

The self-signed toggle is intentionally explicit and should never be enabled in production. Production should use a trusted CA certificate so Node and browsers can verify the peer normally.

## Local mTLS Concept

For local mTLS, create a development CA, issue one server certificate for the backend proxy, and one client certificate for the Next.js server. Configure the proxy to require client certificates and configure the Next.js server-side fetch adapter to present the client certificate.

The assessment keeps this as documentation because production mTLS is usually handled by a service mesh. That avoids duplicating certificate issuance, rotation, and revocation logic across every application.

## Kubernetes mTLS

In Kubernetes, use Istio or Linkerd. The frontend server and Rust backend each receive a workload identity. The mesh sidecar or ambient dataplane establishes mTLS automatically and rotates certificates. Application code still performs authorization because mTLS proves service identity, not end-user intent.

## Secure API Consumption

The browser never calls Assignment 1 directly. It calls the Next.js origin. Next.js server code calls the Rust backend using environment variables and, in production, server-side credentials. This reduces CORS exposure and prevents internal service URLs from becoming part of the public API contract.
