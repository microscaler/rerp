//! Durable render outbox (Epic E1 / NFR-DOG-010).
//!
//! At posting time the invoice service stores, in the SAME transaction,
//! a render instruction together with the immutable render model. The
//! artifact materialiser claims that instruction, records attempts and
//! errors on it, and only marks it DONE once the artifact row exists.
//! Operators inspect instructions via GET /v1/render-outbox; posting
//! never depends on the render pipeline or object storage being up.

use chrono::Utc;
use lifeguard::active_model::ActiveModelTrait;
use lifeguard::{ColumnTrait, LifeExecutor, LifeModelTrait, Order, SessionContext};
use rerp_accounting_invoice::models::render_outbox::{
    AccountingRenderOutboxModel, AccountingRenderOutboxRecord, Column as OutboxColumn,
    Entity as OutboxEntity,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::posting::{PostingError, StoredInvoice};
use rerp_accounting_core::InvoiceSnapshot;

pub const PDF_MEDIA_TYPE: &str = "application/pdf";
pub const STATUS_PENDING: &str = "PENDING";
pub const STATUS_IN_PROGRESS: &str = "IN_PROGRESS";
pub const STATUS_DONE: &str = "DONE";
const MAX_ERROR_CHARS: usize = 480;
/// A crashed claimant must not strand an instruction forever: IN_PROGRESS
/// rows become reclaimable once their lease expires.
pub const CLAIM_LEASE_SECONDS: i64 = 300;

/// Snapshot everything the deterministic renderer needs, so the queued
/// instruction is self-contained for inspection and recovery.
pub fn render_model_for(
    snapshot: &InvoiceSnapshot,
    rounding_minor_units: i32,
) -> Result<Value, PostingError> {
    let snapshot = serde_json::to_value(snapshot)
        .map_err(|error| PostingError::Database(format!("render model: {error}")))?;
    Ok(json!({
        "snapshot": snapshot,
        "rounding_minor_units": rounding_minor_units,
    }))
}

pub fn stored_from_render_model(model: &Value) -> Result<StoredInvoice, PostingError> {
    let snapshot: InvoiceSnapshot = serde_json::from_value(
        model
            .get("snapshot")
            .cloned()
            .ok_or_else(|| PostingError::Database("render model has no snapshot".to_string()))?,
    )
    .map_err(|error| PostingError::Database(format!("render model snapshot: {error}")))?;
    let rounding_minor_units = model
        .get("rounding_minor_units")
        .and_then(Value::as_i64)
        .and_then(|value| i32::try_from(value).ok())
        .ok_or_else(|| {
            PostingError::Database("render model has no rounding_minor_units".to_string())
        })?;
    Ok(StoredInvoice {
        snapshot,
        rounding_minor_units,
    })
}

/// True when an instruction may be (re)attempted. A DONE instruction with
/// a missing artifact is legitimately re-claimable (self-heal). A
/// stranded IN_PROGRESS instruction becomes reclaimable once its lease
/// expires (crashed claimant, lost failure write).
pub fn claimable(status: &str, lease_expired: bool) -> bool {
    status != STATUS_IN_PROGRESS || lease_expired
}

/// Pure lease decision: IN_PROGRESS rows whose `updated_at` is older than
/// the lease may be reclaimed.
pub fn lease_expired(updated_at: chrono::NaiveDateTime, now: chrono::NaiveDateTime) -> bool {
    (now - updated_at).num_seconds() > CLAIM_LEASE_SECONDS
}

/// Truncating, single-line error note for the operator-visible column.
pub fn error_note(message: &str) -> String {
    let flattened = message.replace(['\n', '\r'], " ");
    if flattened.chars().count() <= MAX_ERROR_CHARS {
        return flattened;
    }
    let mut kept: String = flattened.chars().take(MAX_ERROR_CHARS).collect();
    kept.push('…');
    kept
}

/// Atomically with the posting: queue the deterministic PDF render.
pub fn enqueue<E: LifeExecutor>(
    executor: &E,
    snapshot: &InvoiceSnapshot,
    rounding_minor_units: i32,
) -> Result<(), PostingError> {
    let render_model = render_model_for(snapshot, rounding_minor_units)?;
    let now = Utc::now().naive_utc();
    let mut record = AccountingRenderOutboxRecord::new();
    record
        .set_id(Uuid::new_v4())
        .set_tenant_id(snapshot.tenant_id.clone())
        .set_legal_entity_id(snapshot.legal_entity_id)
        .set_document_id(snapshot.id)
        .set_media_type(PDF_MEDIA_TYPE.to_string())
        .set_renderer(crate::pdf::renderer_name().to_string())
        .set_renderer_version(crate::pdf::RENDERER_VERSION.to_string())
        .set_render_model(render_model)
        .set_status(STATUS_PENDING.to_string())
        .set_attempts(0)
        .set_created_at(now)
        .set_updated_at(now);
    match record.insert(executor) {
        Ok(_) => Ok(()),
        Err(error) => {
            // A replayed posting that raced its own idempotency guard may
            // hit the unique (document, media) instruction; the durable
            // queue already has an equivalent entry.
            let text = error.to_string();
            if text.contains("accounting_render_outbox_document_format_unique") {
                Ok(())
            } else {
                Err(PostingError::Database(format!("render outbox: {text}")))
            }
        }
    }
}

/// Documents rendered before the outbox existed still deserve a queue
/// entry: create the DONE instruction whenever an artifact has none.
pub fn ensure_entry_for_artifact<E: LifeExecutor>(
    executor: &E,
    document_id: Uuid,
) -> Result<(), PostingError> {
    let existing = OutboxEntity::find()
        .filter(OutboxColumn::DocumentId.eq(document_id))
        .find_one(executor)
        .map_err(database)?;
    if existing.is_some() {
        return Ok(());
    }
    let invoice = crate::posting::get_invoice(executor, document_id)?;
    backfill_done(executor, &invoice.snapshot, invoice.rounding_minor_units)
}

/// Pre-outbox documents render from the database; record a DONE
/// instruction alongside the artifact so the queue tells the whole truth.
pub fn backfill_done<E: LifeExecutor>(
    executor: &E,
    snapshot: &InvoiceSnapshot,
    rounding_minor_units: i32,
) -> Result<(), PostingError> {
    let render_model = render_model_for(snapshot, rounding_minor_units)?;
    let now = Utc::now().naive_utc();
    let mut record = AccountingRenderOutboxRecord::new();
    record
        .set_id(Uuid::new_v4())
        .set_tenant_id(snapshot.tenant_id.clone())
        .set_legal_entity_id(snapshot.legal_entity_id)
        .set_document_id(snapshot.id)
        .set_media_type(PDF_MEDIA_TYPE.to_string())
        .set_renderer(crate::pdf::renderer_name().to_string())
        .set_renderer_version(crate::pdf::RENDERER_VERSION.to_string())
        .set_render_model(render_model)
        .set_status(STATUS_DONE.to_string())
        .set_attempts(1)
        .set_created_at(now)
        .set_updated_at(now);
    match record.insert(executor) {
        Ok(_) => Ok(()),
        Err(error) => {
            let text = error.to_string();
            if text.contains("accounting_render_outbox_document_format_unique") {
                Ok(())
            } else {
                Err(PostingError::Database(format!("render outbox: {text}")))
            }
        }
    }
}

/// What the artifact materialiser needs to attempt a render right now.
pub enum Claim {
    /// Instruction claimed (attempt counted); render from its model.
    Claimed { id: Uuid, render_model: Value },
    /// Pre-outbox document: no instruction exists; render from the
    /// database and backfill a DONE instruction with the artifact.
    Legacy,
    /// Another attempt is in flight; the caller must retry later.
    InProgress,
}

pub fn claim<E: LifeExecutor>(
    executor: &E,
    legal_entity_hint: Option<Uuid>,
    document_id: Uuid,
) -> Result<Claim, PostingError> {
    let mut query = OutboxEntity::find().filter(OutboxColumn::DocumentId.eq(document_id));
    if let Some(legal_entity_id) = legal_entity_hint {
        query = query.filter(OutboxColumn::LegalEntityId.eq(legal_entity_id));
    }
    let Some(entry) = query.find_one(executor).map_err(database)? else {
        return Ok(Claim::Legacy);
    };
    let expired = lease_expired(entry.updated_at, Utc::now().naive_utc());
    if !claimable(&entry.status, expired) {
        return Ok(Claim::InProgress);
    }
    let mut changes = AccountingRenderOutboxRecord::new();
    changes
        .set_id(entry.id)
        .set_status(STATUS_IN_PROGRESS.to_string())
        .set_attempts(entry.attempts + 1)
        .set_updated_at(Utc::now().naive_utc());
    changes.update(executor).map_err(database)?;
    Ok(Claim::Claimed {
        id: entry.id,
        render_model: entry.render_model.clone(),
    })
}

pub fn record_failure<E: LifeExecutor>(
    executor: &E,
    id: Uuid,
    message: &str,
) -> Result<(), PostingError> {
    let mut changes = AccountingRenderOutboxRecord::new();
    changes
        .set_id(id)
        .set_status(STATUS_PENDING.to_string())
        .set_last_error(Some(error_note(message)))
        .set_updated_at(Utc::now().naive_utc());
    changes.update(executor).map_err(database)?;
    Ok(())
}

pub fn mark_done<E: LifeExecutor>(executor: &E, id: Uuid) -> Result<(), PostingError> {
    let mut changes = AccountingRenderOutboxRecord::new();
    changes
        .set_id(id)
        .set_status(STATUS_DONE.to_string())
        .set_last_error(None)
        .set_updated_at(Utc::now().naive_utc());
    changes.update(executor).map_err(database)?;
    Ok(())
}

pub struct OutboxListFilter {
    pub status: Option<String>,
    pub document_id: Option<Uuid>,
    pub page: u32,
    pub limit: u32,
}

impl OutboxListFilter {
    pub fn parse(
        status: Option<&str>,
        document_id: Option<&str>,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Self, PostingError> {
        let page = page.unwrap_or(1);
        let limit = limit.unwrap_or(20);
        if page < 1 {
            return Err(PostingError::Validation(
                "page must be at least 1".to_string(),
            ));
        }
        if !(1..=100).contains(&limit) {
            return Err(PostingError::Validation(
                "limit must be between 1 and 100".to_string(),
            ));
        }
        let statuses = [STATUS_PENDING, STATUS_IN_PROGRESS, STATUS_DONE];
        if let Some(status) = status {
            if !statuses.contains(&status) {
                return Err(PostingError::Validation(
                    "status must be PENDING, IN_PROGRESS or DONE".to_string(),
                ));
            }
        }
        Ok(Self {
            status: status.map(str::to_string),
            document_id: document_id
                .map(|value| crate::posting::parse_uuid(value, "document_id"))
                .transpose()?,
            page: page as u32,
            limit: limit as u32,
        })
    }
}

/// Operator-visible queue state, oldest first. The render model itself is
/// intentionally not returned.
pub fn list_outbox<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    filter: &OutboxListFilter,
) -> Result<Value, PostingError> {
    let mut query =
        OutboxEntity::find().filter(OutboxColumn::TenantId.eq(session.tenant_id.clone()));
    if let Some(status) = &filter.status {
        query = query.filter(OutboxColumn::Status.eq(status.clone()));
    }
    if let Some(document_id) = filter.document_id {
        query = query.filter(OutboxColumn::DocumentId.eq(document_id));
    }
    let total = query.clone().count().one(executor).map_err(database)?;
    // i64 math: page*limit can overflow u32 for large valid pages.
    let offset = i64::from(filter.page - 1) * i64::from(filter.limit);
    let entries = query
        .order_by(OutboxColumn::CreatedAt, Order::Asc)
        .order_by(OutboxColumn::Id, Order::Asc)
        .limit(u64::from(filter.limit))
        .offset(u64::try_from(offset).unwrap_or(u64::MAX))
        .all(executor)
        .map_err(database)?;
    let items: Vec<Value> = entries.iter().map(entry_json).collect();
    let shown = i64::from(filter.page - 1) * i64::from(filter.limit) + items.len() as i64;
    Ok(json!({
        "items": items,
        "page": filter.page,
        "limit": filter.limit,
        "total": total,
        "has_more": shown < total,
    }))
}

pub fn entry_json(entry: &AccountingRenderOutboxModel) -> Value {
    json!({
        "id": entry.id.to_string(),
        "document_id": entry.document_id.to_string(),
        "media_type": entry.media_type,
        "renderer": entry.renderer,
        "renderer_version": entry.renderer_version,
        "status": entry.status,
        "attempts": entry.attempts,
        "last_error": entry.last_error,
        "created_at": entry.created_at.and_utc().to_rfc3339(),
        "updated_at": entry.updated_at.and_utc().to_rfc3339(),
    })
}

fn database(error: impl std::fmt::Display) -> PostingError {
    PostingError::Database(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claim_state_machine_rejects_fresh_in_progress_only() {
        assert!(claimable(STATUS_PENDING, false));
        assert!(claimable(STATUS_DONE, false));
        assert!(!claimable(STATUS_IN_PROGRESS, false));
        // A stranded claim becomes reclaimable once the lease expires.
        assert!(claimable(STATUS_IN_PROGRESS, true));
    }

    #[test]
    fn claim_lease_boundary_is_strictly_after_the_window() {
        let base = chrono::DateTime::from_timestamp(1_700_000_000, 0)
            .expect("time")
            .naive_utc();
        assert!(!lease_expired(
            base,
            base + chrono::Duration::seconds(CLAIM_LEASE_SECONDS)
        ));
        assert!(lease_expired(
            base,
            base + chrono::Duration::seconds(CLAIM_LEASE_SECONDS + 1)
        ));
    }

    #[test]
    fn error_note_is_single_line_and_bounded() {
        let note = error_note("render failed\nline two\rwith more text");
        assert!(!note.contains('\n') && !note.contains('\r'));
        let long = error_note(&"x".repeat(MAX_ERROR_CHARS + 100));
        assert!(long.chars().count() <= MAX_ERROR_CHARS + 1);
        assert!(long.ends_with('…'));
    }

    #[test]
    fn render_model_reports_missing_or_invalid_snapshot() {
        let error = stored_from_render_model(&json!({}))
            .err()
            .expect("missing snapshot is an error");
        assert!(error.to_string().contains("no snapshot"));
        let error = stored_from_render_model(&json!({"snapshot": {}}))
            .err()
            .expect("invalid snapshot is an error");
        assert!(error.to_string().contains("render model snapshot"));
    }

    #[test]
    fn list_filter_parses_and_rejects_bad_input() {
        let filter =
            OutboxListFilter::parse(Some("PENDING"), None, None, Some(50)).expect("valid filter");
        assert_eq!(filter.page, 1);
        assert_eq!(filter.limit, 50);
        assert!(OutboxListFilter::parse(Some("BOGUS"), None, None, None).is_err());
        assert!(OutboxListFilter::parse(None, Some("not-a-uuid"), None, None).is_err());
        assert!(OutboxListFilter::parse(None, None, Some(0), None).is_err());
        assert!(OutboxListFilter::parse(None, None, None, Some(101)).is_err());
    }
}
