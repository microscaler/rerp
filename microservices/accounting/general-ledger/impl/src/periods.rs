//! Fiscal-period lifecycle: creation and the state machine (Epic D2).
//!
//! Posting into a period is allowed exactly when the period is OPEN or
//! REOPENED. The transition matrix is closed: any transition outside it is
//! a conflict, so period semantics never depend on caller-side conventions.

use chrono::{NaiveDate, NaiveDateTime, Utc};
use lifeguard::active_model::ActiveModelTrait;
use lifeguard::{ColumnTrait, LifeExecutor, LifeModelTrait, SessionContext};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use rerp_accounting_general_ledger_gen::handlers::create_fiscal_period::Request as CreateRequest;
use rerp_accounting_general_ledger_gen::handlers::transition_fiscal_period::Request as TransitionRequest;
use rerp_entities::accounting::foundation::audit_event::AccountingAuditEventRecord;
use rerp_entities::accounting::foundation::fiscal_period::{
    AccountingFiscalPeriodModel as FiscalPeriodModel, AccountingFiscalPeriodRecord,
    Column as FiscalPeriodColumn, Entity as FiscalPeriodEntity,
};
use rerp_entities::accounting::foundation::idempotency_record::AccountingIdempotencyRecordRecord;

use crate::accounts::{find_idempotency, hex_digest, insert, SOURCE_SYSTEM};
use crate::ledger::{parse_date, parse_uuid, resolve_legal_entity, LedgerError};

const SOURCE_TYPE_PERIOD: &str = "fiscal_period";
const TRANSITIONS: &[(&str, &str)] = &[
    ("OPEN", "CLOSED"),
    ("CLOSED", "REOPENED"),
    ("CLOSED", "POSTING_LOCKED"),
    ("REOPENED", "CLOSED"),
    ("REOPENED", "POSTING_LOCKED"),
    ("POSTING_LOCKED", "PERMANENTLY_CLOSED"),
];

pub struct PeriodChange {
    pub period: Value,
    pub created: bool,
}

pub fn period_json(period: &FiscalPeriodModel) -> Value {
    json!({
        "id": period.id.to_string(),
        "name": period.name,
        "start_date": period.start_date.to_string(),
        "end_date": period.end_date.to_string(),
        "state": period.state,
        "closed_at": period.closed_at.map(|value| value.to_string()),
        "closed_by": period.closed_by.map(|value| value.to_string()),
    })
}

pub fn create_fiscal_period<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    request: &CreateRequest,
) -> Result<PeriodChange, LedgerError> {
    let start = parse_date(&request.start_date, "start_date")?;
    let end = parse_date(&request.end_date, "end_date")?;
    if start > end {
        return Err(LedgerError::Validation(
            "start_date must not be after end_date".to_string(),
        ));
    }
    let name = request.name.trim();
    if name.is_empty() || name.len() > 100 {
        return Err(LedgerError::Validation(
            "name must be 1-100 characters".to_string(),
        ));
    }
    let legal_entity = resolve_legal_entity(executor, session)?;
    let fingerprint =
        create_period_fingerprint(session.tenant_id.as_str(), legal_entity.id, request);
    if let Some(record) = find_idempotency(executor, legal_entity.id, &request.idempotency_key)? {
        if record.request_fingerprint != fingerprint {
            return Err(LedgerError::Conflict(
                "idempotency_conflict".to_string(),
                "the idempotency key was already used with a different request body".to_string(),
            ));
        }
        let period = find_by_name(executor, legal_entity.id, name)?.ok_or(LedgerError::NotFound)?;
        return Ok(PeriodChange {
            period: period_json(&period),
            created: false,
        });
    }
    if overlaps(executor, legal_entity.id, start, end)? {
        return Err(LedgerError::Conflict(
            "period_overlap".to_string(),
            "the date range overlaps an existing fiscal period of this legal entity".to_string(),
        ));
    }
    if find_by_name(executor, legal_entity.id, name)?.is_some() {
        return Err(LedgerError::Conflict(
            "period_name_taken".to_string(),
            "a fiscal period with this name already exists for this legal entity".to_string(),
        ));
    }

    let now = Utc::now().naive_utc();
    let period_id = Uuid::new_v4();
    let mut period = AccountingFiscalPeriodRecord::new();
    period
        .set_id(period_id)
        .set_tenant_id(session.tenant_id.clone())
        .set_legal_entity_id(legal_entity.id)
        .set_name(name.to_string())
        .set_start_date(start)
        .set_end_date(end)
        .set_state("OPEN".to_string())
        .set_created_at(now)
        .set_updated_at(now);
    insert(&mut period, executor, "fiscal period")?;

    record_audit(
        executor,
        session,
        legal_entity.id,
        "FISCAL_PERIOD_CREATED",
        period_id,
        &fingerprint,
        now,
    )?;
    record_idempotency(
        executor,
        session,
        legal_entity.id,
        &request.idempotency_key,
        &fingerprint,
        name,
        period_id,
        now,
    )?;
    let period = load_period(executor, legal_entity.id, period_id)?;
    Ok(PeriodChange {
        period: period_json(&period),
        created: true,
    })
}

pub fn transition_fiscal_period<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    request: &TransitionRequest,
) -> Result<PeriodChange, LedgerError> {
    let period_id = parse_uuid(&request.id, "id")?;
    let legal_entity = resolve_legal_entity(executor, session)?;
    let fingerprint =
        transition_period_fingerprint(session.tenant_id.as_str(), legal_entity.id, request);
    if let Some(record) = find_idempotency(executor, legal_entity.id, &request.idempotency_key)? {
        if record.request_fingerprint != fingerprint {
            return Err(LedgerError::Conflict(
                "idempotency_conflict".to_string(),
                "the idempotency key was already used with a different request body".to_string(),
            ));
        }
        let period = load_period(executor, legal_entity.id, period_id)?;
        return Ok(PeriodChange {
            period: period_json(&period),
            created: false,
        });
    }

    // Take the per-period posting lock BEFORE reading state: posting
    // paths hold the same lock across their eligibility check and
    // persist, so a close and a posting can never both believe they won.
    crate::journals::period_posting_lock(executor, period_id)?;
    let period = load_period(executor, legal_entity.id, period_id)?;
    if request.target_state == "REOPENED" {
        let reason = request.reason.as_deref().unwrap_or("").trim();
        if reason.len() < 3 {
            return Err(LedgerError::Validation(
                "reopening a period requires an explicit reason of at least 3 characters"
                    .to_string(),
            ));
        }
    }
    if !transition_allowed(&period.state, &request.target_state) {
        return Err(LedgerError::Conflict(
            "illegal_period_transition".to_string(),
            format!(
                "transition {} -> {} is not allowed",
                period.state, request.target_state
            ),
        ));
    }

    let now = Utc::now().naive_utc();
    let closing = matches!(
        request.target_state.as_str(),
        "CLOSED" | "POSTING_LOCKED" | "PERMANENTLY_CLOSED"
    );
    let mut changes = AccountingFiscalPeriodRecord::new();
    changes
        .set_id(period_id)
        .set_state(request.target_state.clone())
        .set_updated_at(now);
    if closing {
        changes
            .set_closed_at(Some(now))
            .set_closed_by(Some(session.subject_id));
    } else {
        changes.set_closed_at(None).set_closed_by(None);
    }
    changes
        .update(executor)
        .map_err(|error| LedgerError::Database(format!("fiscal period transition: {error}")))?;

    record_audit(
        executor,
        session,
        legal_entity.id,
        &format!("FISCAL_PERIOD_{}", request.target_state),
        period_id,
        &fingerprint,
        now,
    )?;
    record_idempotency(
        executor,
        session,
        legal_entity.id,
        &request.idempotency_key,
        &fingerprint,
        &format!("{period_id}:{}", request.target_state),
        period_id,
        now,
    )?;

    let updated = load_period(executor, legal_entity.id, period_id)?;
    Ok(PeriodChange {
        period: period_json(&updated),
        created: false,
    })
}

fn transition_allowed(from: &str, to: &str) -> bool {
    TRANSITIONS.contains(&(from, to))
}

fn load_period<E: LifeExecutor>(
    executor: &E,
    legal_entity_id: Uuid,
    period_id: Uuid,
) -> Result<FiscalPeriodModel, LedgerError> {
    FiscalPeriodEntity::find()
        .filter(FiscalPeriodColumn::LegalEntityId.eq(legal_entity_id))
        .filter(FiscalPeriodColumn::Id.eq(period_id))
        .find_one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?
        .ok_or(LedgerError::NotFound)
}

fn find_by_name<E: LifeExecutor>(
    executor: &E,
    legal_entity_id: Uuid,
    name: &str,
) -> Result<Option<FiscalPeriodModel>, LedgerError> {
    FiscalPeriodEntity::find()
        .filter(FiscalPeriodColumn::LegalEntityId.eq(legal_entity_id))
        .filter(FiscalPeriodColumn::Name.eq(name))
        .find_one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))
}

fn overlaps<E: LifeExecutor>(
    executor: &E,
    legal_entity_id: Uuid,
    start: NaiveDate,
    end: NaiveDate,
) -> Result<bool, LedgerError> {
    let matching = FiscalPeriodEntity::find()
        .filter(FiscalPeriodColumn::LegalEntityId.eq(legal_entity_id))
        .filter(FiscalPeriodColumn::StartDate.lte(end))
        .filter(FiscalPeriodColumn::EndDate.gte(start))
        .count()
        .one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;
    Ok(matching > 0)
}

fn record_audit<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    legal_entity_id: Uuid,
    action: &str,
    period_id: Uuid,
    fingerprint: &str,
    now: NaiveDateTime,
) -> Result<(), LedgerError> {
    let mut audit = AccountingAuditEventRecord::new();
    audit
        .set_id(Uuid::new_v4())
        .set_tenant_id(session.tenant_id.clone())
        .set_legal_entity_id(legal_entity_id)
        .set_subject_id(session.subject_id)
        .set_action(action.to_string())
        .set_document_id(period_id)
        .set_source_system(SOURCE_SYSTEM.to_string())
        .set_request_fingerprint(fingerprint.to_string())
        .set_occurred_at(now);
    insert(&mut audit, executor, "audit event")?;
    Ok(())
}

fn record_idempotency<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    legal_entity_id: Uuid,
    idempotency_key: &str,
    fingerprint: &str,
    source_id: &str,
    document_id: Uuid,
    now: NaiveDateTime,
) -> Result<(), LedgerError> {
    let mut idempotency = AccountingIdempotencyRecordRecord::new();
    idempotency
        .set_id(Uuid::new_v4())
        .set_tenant_id(session.tenant_id.clone())
        .set_legal_entity_id(legal_entity_id)
        .set_idempotency_key(idempotency_key.to_string())
        .set_request_fingerprint(fingerprint.to_string())
        .set_status("COMPLETED".to_string())
        .set_source_system(SOURCE_SYSTEM.to_string())
        .set_source_type(SOURCE_TYPE_PERIOD.to_string())
        .set_source_id(source_id.to_string())
        .set_document_id(Some(document_id))
        .set_created_at(now)
        .set_completed_at(Some(now));
    insert(&mut idempotency, executor, "idempotency record")?;
    Ok(())
}

fn create_period_fingerprint(
    tenant_id: &str,
    legal_entity_id: Uuid,
    request: &CreateRequest,
) -> String {
    let parts: Vec<Vec<u8>> = vec![
        tenant_id.as_bytes().to_vec(),
        legal_entity_id.as_bytes().to_vec(),
        b"CREATE_PERIOD".to_vec(),
        request.name.trim().as_bytes().to_vec(),
        request.start_date.as_bytes().to_vec(),
        request.end_date.as_bytes().to_vec(),
    ];
    digest_parts(parts)
}

fn transition_period_fingerprint(
    tenant_id: &str,
    legal_entity_id: Uuid,
    request: &TransitionRequest,
) -> String {
    let mut parts: Vec<Vec<u8>> = vec![
        tenant_id.as_bytes().to_vec(),
        legal_entity_id.as_bytes().to_vec(),
        b"TRANSITION_PERIOD".to_vec(),
        request.id.trim().as_bytes().to_vec(),
        request.target_state.as_bytes().to_vec(),
    ];
    if let Some(reason) = &request.reason {
        parts.push(reason.trim().as_bytes().to_vec());
    }
    digest_parts(parts)
}

fn digest_parts(parts: Vec<Vec<u8>>) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update((part.len() as u64).to_be_bytes());
        hasher.update(part);
    }
    hex_digest(&hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transition_matrix_is_closed() {
        for (from, to) in TRANSITIONS {
            assert!(transition_allowed(from, to), "{from} -> {to}");
        }
        for (from, to) in [
            ("OPEN", "REOPENED"),
            ("OPEN", "POSTING_LOCKED"),
            ("OPEN", "PERMANENTLY_CLOSED"),
            ("CLOSED", "OPEN"),
            ("REOPENED", "OPEN"),
            ("POSTING_LOCKED", "REOPENED"),
            ("POSTING_LOCKED", "CLOSED"),
            ("PERMANENTLY_CLOSED", "REOPENED"),
            ("PERMANENTLY_CLOSED", "CLOSED"),
        ] {
            assert!(!transition_allowed(from, to), "{from} -> {to}");
        }
    }

    #[test]
    fn create_period_fingerprint_binds_dates() {
        fn create(name: &str, start: &str, end: &str) -> CreateRequest {
            CreateRequest {
                idempotency_key: "period-create-0001".to_string(),
                name: name.to_string(),
                start_date: start.to_string(),
                end_date: end.to_string(),
            }
        }
        let entity = Uuid::new_v4();
        let year = create("FY2026", "2026-01-01", "2026-12-31");
        let half = create("FY2026", "2026-01-01", "2026-06-30");
        let renamed = create("FY-2026", "2026-01-01", "2026-12-31");
        assert_ne!(
            create_period_fingerprint("hauliage", entity, &year),
            create_period_fingerprint("hauliage", entity, &half),
        );
        assert_ne!(
            create_period_fingerprint("hauliage", entity, &year),
            create_period_fingerprint("hauliage", entity, &renamed),
        );
    }

    #[test]
    fn transition_fingerprint_binds_state_and_reason() {
        let entity = Uuid::new_v4();
        let period = Uuid::new_v4();
        fn transition(period: &Uuid, state: &str, reason: Option<&str>) -> TransitionRequest {
            TransitionRequest {
                id: period.to_string(),
                idempotency_key: "period-transition-0001".to_string(),
                target_state: state.to_string(),
                reason: reason.map(str::to_string),
            }
        }
        let close = transition(&period, "CLOSED", None);
        let reopen = transition(&period, "REOPENED", Some("quarter re-audit"));
        let reopen_other_reason = transition(&period, "REOPENED", Some("audit exception"));
        assert_ne!(
            transition_period_fingerprint("hauliage", entity, &close),
            transition_period_fingerprint("hauliage", entity, &reopen),
        );
        assert_ne!(
            transition_period_fingerprint("hauliage", entity, &reopen),
            transition_period_fingerprint("hauliage", entity, &reopen_other_reason),
        );
        assert_ne!(
            transition_period_fingerprint("hauliage", entity, &close),
            create_period_fingerprint(
                "hauliage",
                entity,
                &CreateRequest {
                    idempotency_key: "k".to_string(),
                    name: "x".to_string(),
                    start_date: "2026-01-01".to_string(),
                    end_date: "2026-12-31".to_string(),
                }
            ),
        );
    }

    #[test]
    #[ignore = "requires a disposable PostgreSQL database with the RERP foundation migrations"]
    fn live_create_replay_overlap_close_and_reopen_lifecycle() {
        let session = SessionContext {
            tenant_id: "hauliage".to_string(),
            subject_id: Uuid::parse_str("a1000001-0001-4000-8000-000000000004")
                .expect("subject UUID"),
            organization_id: Uuid::parse_str("b2000002-0002-4000-8000-000000000002")
                .expect("organization UUID"),
            session_id: "runtime-acceptance".to_string(),
            roles: vec!["finance-writer".to_string()],
            permissions: vec![
                "accounting:ledger:read".to_string(),
                "accounting:ledger:write".to_string(),
            ],
            user_type: Some("service".to_string()),
            org_type: Some("shipper".to_string()),
        };
        fn transition(id: &str, key: &str, state: &str, reason: Option<&str>) -> TransitionRequest {
            TransitionRequest {
                id: id.to_string(),
                idempotency_key: key.to_string(),
                target_state: state.to_string(),
                reason: reason.map(str::to_string),
            }
        }
        let first = CreateRequest {
            idempotency_key: "period-live-0001".to_string(),
            name: "LIVE FY2026".to_string(),
            start_date: "2026-01-01".to_string(),
            end_date: "2026-12-31".to_string(),
        };
        let created = crate::http_support::with_accounting_transaction(&session, |executor| {
            create_fiscal_period(executor, &session, &first)
        })
        .expect("period creation");
        assert!(created.created);
        assert_eq!(created.period["state"], "OPEN");
        let period_id = created.period["id"]
            .as_str()
            .expect("period id")
            .to_string();

        let replay = crate::http_support::with_accounting_transaction(&session, |executor| {
            create_fiscal_period(executor, &session, &first)
        })
        .expect("idempotent replay");
        assert!(!replay.created);
        assert_eq!(replay.period["id"], created.period["id"]);

        // Fresh key, overlapping range must conflict.
        let overlapping = CreateRequest {
            idempotency_key: "period-live-0002".to_string(),
            name: "LIVE FY2026 Q1".to_string(),
            start_date: "2026-01-01".to_string(),
            end_date: "2026-03-31".to_string(),
        };
        let overlap = crate::http_support::with_accounting_transaction(&session, |executor| {
            create_fiscal_period(executor, &session, &overlapping)
        });
        assert!(matches!(
            overlap,
            Err(LedgerError::Conflict(ref code, _)) if code == "period_overlap"
        ));

        let close = transition(&period_id, "period-live-0003", "CLOSED", None);
        let closed = crate::http_support::with_accounting_transaction(&session, |executor| {
            transition_fiscal_period(executor, &session, &close)
        })
        .expect("close");
        assert_eq!(closed.period["state"], "CLOSED");
        assert!(closed.period["closed_at"].is_string());

        // Reopen without a reason is rejected; with a reason it succeeds.
        let reasonless = transition(&period_id, "period-live-0004", "REOPENED", None);
        let rejected = crate::http_support::with_accounting_transaction(&session, |executor| {
            transition_fiscal_period(executor, &session, &reasonless)
        });
        assert!(matches!(rejected, Err(LedgerError::Validation(_))));

        let reopen = transition(
            &period_id,
            "period-live-0005",
            "REOPENED",
            Some("quarter re-audit"),
        );
        let reopened = crate::http_support::with_accounting_transaction(&session, |executor| {
            transition_fiscal_period(executor, &session, &reopen)
        })
        .expect("reopen with reason");
        assert_eq!(reopened.period["state"], "REOPENED");
        assert!(reopened.period["closed_at"].is_null());

        // Illegal from REOPENED: straight to PERMANENTLY_CLOSED.
        let illegal = transition(&period_id, "period-live-0006", "PERMANENTLY_CLOSED", None);
        let conflict = crate::http_support::with_accounting_transaction(&session, |executor| {
            transition_fiscal_period(executor, &session, &illegal)
        });
        assert!(matches!(
            conflict,
            Err(LedgerError::Conflict(ref code, _)) if code == "illegal_period_transition"
        ));
    }
}
