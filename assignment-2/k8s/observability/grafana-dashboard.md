# Grafana Dashboard Panels

Create panels for:

- Request rate by route and status.
- p50/p95/p99 latency by route.
- Certificate issuance success and failure count.
- Certificates expiring in 7, 14, and 30 days.
- PostgreSQL pool utilization.
- Signing backend latency and failures.
- Audit write failures.

The dashboard should separate user-facing SLO signals from dependency diagnostics. This keeps incident response focused: first determine whether users are impacted, then drill into database, CA, or mesh behavior.
