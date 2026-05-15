use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::Certificate;
use uuid::Uuid;

pub async fn insert_certificate(pool: &DbPool, certificate: &Certificate) -> Result<(), AppError> {
    // This dual-path pattern lets us choose compile-time SQL verification in
    // stricter CI builds while preserving fast local iteration when desired.
    #[cfg(sqlx_checked)]
    {
        sqlx::query!(
            r#"INSERT INTO certificates (certificate_id, subject, issuer, expiration, san_entries)
               VALUES ($1, $2, $3, $4, $5)"#,
            certificate.certificate_id,
            certificate.subject,
            certificate.issuer,
            certificate.expiration,
            &certificate.san_entries,
        )
        .execute(pool)
        .await
        .map_err(AppError::Db)?;
    }

    #[cfg(not(sqlx_checked))]
    {
        sqlx::query(
            r#"INSERT INTO certificates (certificate_id, subject, issuer, expiration, san_entries)
               VALUES ($1, $2, $3, $4, $5)"#,
        )
        .bind(certificate.certificate_id)
        .bind(&certificate.subject)
        .bind(&certificate.issuer)
        .bind(certificate.expiration)
        .bind(&certificate.san_entries)
        .execute(pool)
        .await
        .map_err(AppError::Db)?;
    }

    Ok(())
}

pub async fn find_certificate_by_id(
    pool: &DbPool,
    certificate_id: Uuid,
) -> Result<Option<Certificate>, AppError> {
    // Repositories return Option for lookup APIs instead of forcing HTTP-ish
    // error decisions at this layer. Upstream layers decide 404 semantics.
    #[cfg(sqlx_checked)]
    {
        let certificate = sqlx::query_as!(
            Certificate,
            r#"SELECT certificate_id, subject, issuer, expiration, san_entries FROM certificates WHERE certificate_id = $1"#,
            certificate_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(AppError::Db)?;

        Ok(certificate)
    }

    #[cfg(not(sqlx_checked))]
    {
        let certificate = sqlx::query_as::<_, Certificate>(
            r#"SELECT certificate_id, subject, issuer, expiration, san_entries FROM certificates WHERE certificate_id = $1"#,
        )
        .bind(certificate_id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Db)?;

        Ok(certificate)
    }
}

pub async fn list_certificates(pool: &DbPool) -> Result<Vec<Certificate>, AppError> {
    // Inventory callers need a real list endpoint instead of knowing IDs ahead
    // of time. Expiration ordering surfaces soonest-renewal items first.
    #[cfg(sqlx_checked)]
    {
        let certificates = sqlx::query_as!(
            Certificate,
            r#"SELECT certificate_id, subject, issuer, expiration, san_entries FROM certificates ORDER BY expiration ASC"#
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::Db)?;

        Ok(certificates)
    }

    #[cfg(not(sqlx_checked))]
    {
        let certificates = sqlx::query_as::<_, Certificate>(
            r#"SELECT certificate_id, subject, issuer, expiration, san_entries FROM certificates ORDER BY expiration ASC"#,
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::Db)?;

        Ok(certificates)
    }
}
