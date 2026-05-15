-- PostgreSQL schema for a production certificate inventory service.
-- The design intentionally separates issuance requests, final certificate inventory,
-- and audit evidence. This allows failed requests to be investigated without
-- pretending that every request produced a certificate.

CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TYPE certificate_status AS ENUM ('active', 'revoked', 'expired');
CREATE TYPE issuance_status AS ENUM ('pending', 'issued', 'rejected', 'failed');

CREATE TABLE tenants (
    tenant_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE certificate_issuance_requests (
    request_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants (tenant_id),
    requested_by TEXT NOT NULL,
    csr_pem TEXT NOT NULL,
    requested_subject TEXT NOT NULL,
    requested_sans TEXT[] NOT NULL CHECK (cardinality(requested_sans) > 0),
    status issuance_status NOT NULL DEFAULT 'pending',
    rejection_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at TIMESTAMPTZ
);

CREATE TABLE certificates (
    certificate_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants (tenant_id),
    request_id UUID NOT NULL UNIQUE REFERENCES certificate_issuance_requests (request_id),
    serial_number TEXT NOT NULL UNIQUE,
    subject TEXT NOT NULL,
    issuer TEXT NOT NULL,
    san_entries TEXT[] NOT NULL CHECK (cardinality(san_entries) > 0),
    not_before TIMESTAMPTZ NOT NULL,
    not_after TIMESTAMPTZ NOT NULL,
    status certificate_status NOT NULL DEFAULT 'active',
    certificate_pem TEXT NOT NULL,
    fingerprint_sha256 TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    revoked_at TIMESTAMPTZ,
    revocation_reason TEXT,
    CHECK (not_after > not_before)
);

CREATE TABLE audit_events (
    audit_event_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID,
    actor TEXT NOT NULL,
    action TEXT NOT NULL,
    target_type TEXT NOT NULL,
    target_id TEXT,
    decision TEXT NOT NULL,
    request_id UUID,
    source_workload TEXT,
    source_ip INET,
    details JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Inventory screens and renewal jobs usually query by expiration and status.
CREATE INDEX idx_certificates_tenant_status_expiry
    ON certificates (tenant_id, status, not_after);

-- Subject search is a common operator workflow.
CREATE INDEX idx_certificates_subject
    ON certificates (subject);

-- SAN filtering benefits from a GIN index because SANs are represented as an array.
CREATE INDEX idx_certificates_sans_gin
    ON certificates USING GIN (san_entries);

-- Audit timelines are tenant/time oriented and should remain fast at scale.
CREATE INDEX idx_audit_events_tenant_created_at
    ON audit_events (tenant_id, created_at DESC);

CREATE INDEX idx_issuance_requests_tenant_status_created_at
    ON certificate_issuance_requests (tenant_id, status, created_at DESC);
