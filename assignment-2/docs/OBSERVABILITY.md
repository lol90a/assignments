# Observability

## Logs

The Rust service uses `tracing` for structured logs. Each request should include a request id, route, status code, latency, actor, tenant, and certificate id when available. Structured logs are preferred over free-form strings because they are queryable in systems like Loki, Elasticsearch, or CloudWatch.

## Metrics

Prometheus scrapes `/metrics`. Useful metrics include:

- HTTP request count, latency, and status code.
- Certificate issuance count by status and issuer.
- CSR validation failures by reason.
- Database query latency and pool saturation.
- Signing backend latency and error count.
- Audit write failures.

## Traces

OpenTelemetry spans connect the incoming HTTP request, database transaction, signing backend call, and audit write. This is valuable because issuance latency often comes from external dependencies, not from Rust CPU time.

## Dashboards

Grafana should show traffic, errors, saturation, and latency. For certificate-specific operations, include issuance success rate, certificates expiring soon, signing backend failures, and p95/p99 latencies.

## Alerting

Alert on high 5xx rate, database pool exhaustion, signing backend failures, audit persistence failures, and a spike in rejected certificate policies. Audit failure alerts should page because losing audit evidence can be a compliance incident.
