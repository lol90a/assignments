CREATE TABLE IF NOT EXISTS certificates (
    certificate_id UUID PRIMARY KEY,
    subject TEXT NOT NULL CHECK (length(btrim(subject)) > 0),
    issuer TEXT NOT NULL CHECK (length(btrim(issuer)) > 0),
    expiration TIMESTAMPTZ NOT NULL,
    san_entries TEXT[] NOT NULL CHECK (cardinality(san_entries) > 0)
);

CREATE INDEX IF NOT EXISTS idx_certificates_expiration ON certificates (expiration);
CREATE INDEX IF NOT EXISTS idx_certificates_subject ON certificates (subject);
