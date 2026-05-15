# Production Hardening

## Reliability

Use readiness probes to remove pods from service before they receive traffic and liveness probes to recover wedged processes. Keep liveness checks shallow; they should prove the process is alive, not that every dependency is healthy. Readiness can include database connectivity because a pod that cannot reach PostgreSQL cannot serve issuance requests.

## Performance

The service is mostly I/O-bound. Tokio allows high concurrency while SQLx connection pooling protects PostgreSQL from unbounded connection fan-out. CPU-heavy certificate parsing should be bounded by request size limits and timeouts. Expensive signing operations should have explicit deadlines.

## Scalability

Scale API pods horizontally with HPA based on CPU and request metrics. Scale PostgreSQL separately using managed database capacity, connection pooling, read replicas for inventory queries if needed, and partitioning for very large audit tables.

## Supply Chain

Use multi-stage Docker builds, distroless or minimal runtime images, SBOM generation, vulnerability scanning, signed images, and pinned base image digests. Run `cargo audit`, `cargo deny`, and container scanners in CI.

## Operational Controls

Prefer progressive delivery with rolling updates or canaries. Use PodDisruptionBudgets to avoid voluntary disruption taking down too many pods. Store migrations as code and run them through an explicit migration job rather than letting every pod race to migrate on startup in production.
