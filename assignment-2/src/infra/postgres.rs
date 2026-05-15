use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, PgPool};
use uuid::Uuid;

use crate::domain::{
    model::{IssueCertificateCommand, IssuedCertificate},
    ports::CertificateRepository,
};

pub async fn connect(database_url: &str) -> anyhow::Result<PgPool> {
    // The pool is bounded to protect PostgreSQL. Horizontal pod autoscaling can
    // otherwise multiply connections until the database becomes the bottleneck.
    Ok(PgPoolOptions::new()
        .max_connections(10)
        .min_connections(1)
        .connect(database_url)
        .await?)
}

pub struct PostgresCertificateRepository {
    pool: PgPool,
}

impl PostgresCertificateRepository {
    pub fn new(pool: PgPool) -> Self {
        // The repository owns a cheap cloneable PgPool handle; SQLx manages
        // the actual shared connection pool internally.
        Self { pool }
    }
}

#[async_trait]
impl CertificateRepository for PostgresCertificateRepository {
    async fn persist_issued_certificate(
        &self,
        command: &IssueCertificateCommand,
        certificate: &IssuedCertificate,
    ) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;

        // A transaction keeps request completion, inventory, and audit evidence
        // consistent. In a production codebase these statements would be SQLx
        // query macros tied to checked migrations.
        sqlx::query(
            r#"
            INSERT INTO tenants (tenant_id, name)
            VALUES ($1, $2)
            ON CONFLICT (tenant_id) DO NOTHING
            "#,
        )
        .bind(command.tenant_id)
        .bind(format!("tenant-{}", command.tenant_id))
        .execute(&mut *tx)
        .await?;

        let request_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO certificate_issuance_requests (
                tenant_id, requested_by, csr_pem, requested_subject, requested_sans, status, completed_at
            )
            VALUES ($1, $2, $3, $4, $5, 'issued', now())
            RETURNING request_id
            "#,
        )
        .bind(command.tenant_id)
        .bind(&command.actor)
        .bind(&command.csr_pem)
        .bind(&command.requested_subject)
        .bind(&command.requested_sans)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO certificates (
                certificate_id, tenant_id, request_id, serial_number, subject, issuer,
                san_entries, not_before, not_after, certificate_pem, fingerprint_sha256
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(certificate.certificate_id)
        .bind(command.tenant_id)
        .bind(request_id)
        .bind(&certificate.serial_number)
        .bind(&certificate.subject)
        .bind(&certificate.issuer)
        .bind(&certificate.san_entries)
        .bind(certificate.not_before)
        .bind(certificate.not_after)
        .bind(&certificate.certificate_pem)
        .bind(format!("dummy-{}", certificate.serial_number))
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO audit_events (tenant_id, actor, action, target_type, target_id, decision, request_id, details)
            VALUES ($1, $2, 'certificate.issue', 'certificate', $3, 'allowed', $4, $5)
            "#,
        )
        .bind(command.tenant_id)
        .bind(&command.actor)
        .bind(certificate.certificate_id.to_string())
        .bind(request_id)
        .bind(serde_json::json!({
            "subject": certificate.subject,
            "serial_number": certificate.serial_number
        }))
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn list_certificates(&self) -> anyhow::Result<Vec<IssuedCertificate>> {
        let certificates = sqlx::query_as::<_, IssuedCertificate>(
            r#"
            SELECT certificate_id, serial_number, subject, issuer, san_entries,
                   not_before, not_after, certificate_pem
            FROM certificates
            ORDER BY not_after ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(certificates)
    }

    async fn find_certificate_by_id(
        &self,
        certificate_id: Uuid,
    ) -> anyhow::Result<Option<IssuedCertificate>> {
        let certificate = sqlx::query_as::<_, IssuedCertificate>(
            r#"
            SELECT certificate_id, serial_number, subject, issuer, san_entries,
                   not_before, not_after, certificate_pem
            FROM certificates
            WHERE certificate_id = $1
            "#,
        )
        .bind(certificate_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(certificate)
    }
}
