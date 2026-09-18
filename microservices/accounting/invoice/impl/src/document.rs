//! Materialisation and retrieval of immutable rendered invoice artifacts.

use chrono::Utc;
use lifeguard::active_model::ActiveModelTrait;
use lifeguard::{ColumnTrait, LifeExecutor, LifeModelTrait, SessionContext};
use rerp_entities::accounting::foundation::document_artifact::{
    AccountingDocumentArtifactModel, AccountingDocumentArtifactRecord, Column as ArtifactColumn,
    Entity as ArtifactEntity,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::object_store::{sha256_hex, store};
use crate::posting::{PostingError, StoredInvoice};

const PDF_MEDIA_TYPE: &str = "application/pdf";

pub fn retrieve(context: &SessionContext, document_id: Uuid) -> Result<Value, PostingError> {
    let object_store = store().map_err(PostingError::Unavailable)?;
    let existing = crate::http_support::with_accounting_transaction(context, |executor| {
        load_artifact(executor, document_id)
    })?;
    let artifact = match existing {
        Some(artifact) => artifact,
        None => materialize(context, document_id, object_store)?,
    };
    if artifact.bucket != object_store.bucket() {
        return Err(PostingError::Unavailable(
            "artifact bucket does not match the configured object store".to_string(),
        ));
    }
    let download = object_store
        .presigned_get(&artifact.object_key)
        .map_err(PostingError::Unavailable)?;
    Ok(json!({
        "document_id": artifact.document_id,
        "media_type": artifact.media_type,
        "sha256": artifact.sha256,
        "size_bytes": artifact.size_bytes,
        "renderer": artifact.renderer,
        "renderer_version": artifact.renderer_version,
        "rendered_at": artifact.rendered_at.and_utc().to_rfc3339(),
        "download_url": download.url,
        "expires_at": download.expires_at.to_rfc3339(),
    }))
}

fn materialize(
    context: &SessionContext,
    document_id: Uuid,
    object_store: &crate::object_store::ObjectStore,
) -> Result<AccountingDocumentArtifactModel, PostingError> {
    // Load and commit before rendering: network I/O must never hold a database
    // transaction or a Lifeguard pool slot open.
    let invoice = crate::http_support::with_accounting_transaction(context, |executor| {
        crate::posting::get_invoice(executor, document_id)
    })?;
    let pdf = crate::pdf::render(&invoice);
    let checksum = sha256_hex(&pdf);
    let key = object_key(&invoice, &checksum);
    object_store
        .put_immutable(&key, &pdf, &checksum)
        .map_err(PostingError::Unavailable)?;
    let rendered_at = Utc::now().naive_utc();
    crate::http_support::with_accounting_transaction(context, |executor| {
        if let Some(existing) = load_artifact(executor, document_id)? {
            return compatible(existing, &checksum, pdf.len());
        }
        let snapshot = &invoice.snapshot;
        let mut record = AccountingDocumentArtifactRecord::new();
        record
            .set_id(Uuid::new_v4())
            .set_tenant_id(snapshot.tenant_id.clone())
            .set_legal_entity_id(snapshot.legal_entity_id)
            .set_document_id(snapshot.id)
            .set_media_type(PDF_MEDIA_TYPE.to_string())
            .set_storage_provider("S3_COMPATIBLE".to_string())
            .set_bucket(object_store.bucket().to_string())
            .set_object_key(key.clone())
            .set_sha256(checksum.clone())
            .set_size_bytes(i64::try_from(pdf.len()).map_err(|_| {
                PostingError::Unavailable("rendered document is too large".to_string())
            })?)
            .set_renderer(crate::pdf::renderer_name().to_string())
            .set_renderer_version(crate::pdf::RENDERER_VERSION.to_string())
            .set_rendered_at(rendered_at)
            .set_rendered_by(context.subject_id)
            .set_created_at(rendered_at);
        match record.insert(executor) {
            Ok(model) => Ok(model),
            Err(error) => {
                // A concurrent request may have won the unique document/format
                // insert after both uploaded the same content-addressed object.
                match load_artifact(executor, document_id)? {
                    Some(existing) => compatible(existing, &checksum, pdf.len()),
                    None => Err(PostingError::Database(format!(
                        "document artifact metadata: {error}"
                    ))),
                }
            }
        }
    })
}

fn load_artifact<E: LifeExecutor>(
    executor: &E,
    document_id: Uuid,
) -> Result<Option<AccountingDocumentArtifactModel>, PostingError> {
    ArtifactEntity::find()
        .filter(ArtifactColumn::DocumentId.eq(document_id))
        .filter(ArtifactColumn::MediaType.eq(PDF_MEDIA_TYPE))
        .find_one(executor)
        .map_err(|error| PostingError::Database(error.to_string()))
}

fn compatible(
    artifact: AccountingDocumentArtifactModel,
    checksum: &str,
    size: usize,
) -> Result<AccountingDocumentArtifactModel, PostingError> {
    if artifact.sha256 == checksum && artifact.size_bytes == size as i64 {
        Ok(artifact)
    } else {
        Err(PostingError::Database(
            "immutable document metadata conflicts with deterministic rendering".to_string(),
        ))
    }
}

fn object_key(invoice: &StoredInvoice, checksum: &str) -> String {
    format!(
        "accounting/{}/{}/{}.pdf",
        invoice.snapshot.legal_entity_id, invoice.snapshot.id, checksum
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use rerp_accounting_core::{
        AccountingDocumentStatus, AccountingDocumentType, CurrencyCode, InvoiceSnapshot,
        SourceReference,
    };
    use rust_decimal::Decimal;

    #[test]
    fn object_key_contains_no_tenant_or_customer_data() {
        let invoice = StoredInvoice {
            rounding_minor_units: 2,
            snapshot: InvoiceSnapshot {
                id: Uuid::parse_str("10000000-0000-4000-8000-000000000001").unwrap(),
                tenant_id: "private-tenant-name".to_string(),
                legal_entity_id: Uuid::parse_str("20000000-0000-4000-8000-000000000002").unwrap(),
                fiscal_period_id: Uuid::nil(),
                document_number: "INV-1".to_string(),
                document_type: AccountingDocumentType::CustomerInvoice,
                status: AccountingDocumentStatus::Posted,
                original_document_id: None,
                customer_id: Uuid::parse_str("30000000-0000-4000-8000-000000000003").unwrap(),
                source: SourceReference {
                    system: "source".to_string(),
                    resource_type: "job".to_string(),
                    resource_id: "secret".to_string(),
                },
                invoice_date: NaiveDate::from_ymd_opt(2026, 7, 14).unwrap(),
                due_date: NaiveDate::from_ymd_opt(2026, 8, 14).unwrap(),
                currency: CurrencyCode::try_from("EUR").unwrap(),
                posted_at: NaiveDate::from_ymd_opt(2026, 7, 14)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
                subtotal: Decimal::ONE,
                discount_amount: Decimal::ZERO,
                tax_amount: Decimal::ZERO,
                total_amount: Decimal::ONE,
                lines: vec![],
            },
        };
        let key = object_key(&invoice, &"a".repeat(64));
        assert!(!key.contains("private-tenant-name"));
        assert!(!key.contains("secret"));
        assert!(key.ends_with(".pdf"));
    }
}
