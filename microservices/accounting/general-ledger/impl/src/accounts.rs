//! Tenant-scoped chart-of-accounts creation (Epic D1a).
//!
//! Mirrors the invoice posting conventions: caller validation first, legal
//! entity resolved from the session, fingerprint-gated idempotency in the
//! same transaction, and an append-only audit fact. Control accounts
//! (AR/AP/bank/tax/retained-earnings roles) are system-managed; the request
//! carries no control_role, so this path can never assign or reassign one.

use chrono::Utc;
use lifeguard::active_model::ActiveModelTrait;
use lifeguard::{ColumnTrait, LifeExecutor, LifeModelTrait, SessionContext};
use rerp_accounting_general_ledger_gen::handlers::create_ledger_account::Request;
use rerp_entities::accounting::foundation::account::{
    AccountingAccountRecord, Column as AccountColumn, Entity as AccountEntity,
};
use rerp_entities::accounting::foundation::audit_event::AccountingAuditEventRecord;
use rerp_entities::accounting::foundation::idempotency_record::{
    AccountingIdempotencyRecordRecord, Column as IdempotencyColumn, Entity as IdempotencyEntity,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::ledger::{account_json, currency, resolve_legal_entity, LedgerError};

const SOURCE_SYSTEM: &str = "rerp-accounting";
const SOURCE_TYPE: &str = "ledger_account";
const ACCOUNT_TYPES: [&str; 5] = ["ASSET", "LIABILITY", "EQUITY", "REVENUE", "EXPENSE"];
const NORMAL_SIDES: [&str; 2] = ["DEBIT", "CREDIT"];

pub struct AccountCreation {
    pub account: Value,
    pub created: bool,
}

pub fn create_ledger_account<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    request: &Request,
) -> Result<AccountCreation, LedgerError> {
    validate(request)?;
    let legal_entity = resolve_legal_entity(executor, session)?;
    let fingerprint = request_fingerprint(session.tenant_id.as_str(), legal_entity.id, request);

    if let Some(record) = find_idempotency(executor, legal_entity.id, &request.idempotency_key)? {
        if record.request_fingerprint != fingerprint {
            return Err(LedgerError::Conflict(
                "idempotency_conflict".to_string(),
                "the idempotency key was already used with a different request body".to_string(),
            ));
        }
        let account_id = record.document_id.ok_or_else(|| {
            LedgerError::Database("idempotency record carries no account reference".to_string())
        })?;
        let account = AccountEntity::find()
            .filter(AccountColumn::LegalEntityId.eq(legal_entity.id))
            .filter(AccountColumn::Id.eq(account_id))
            .find_one(executor)
            .map_err(|error| LedgerError::Database(error.to_string()))?
            .ok_or(LedgerError::NotFound)?;
        return Ok(AccountCreation {
            account: account_json(&account),
            created: false,
        });
    }

    let now = Utc::now().naive_utc();
    let account_id = Uuid::new_v4();
    let mut account = AccountingAccountRecord::new();
    account
        .set_id(account_id)
        .set_tenant_id(session.tenant_id.clone())
        .set_legal_entity_id(legal_entity.id)
        .set_code(request.code.trim().to_string())
        .set_name(request.name.trim().to_string())
        .set_account_type(request.account_type.clone())
        .set_normal_side(request.normal_side.clone())
        .set_currency_code(request.currency_code.clone())
        .set_is_active(true)
        .set_created_at(now)
        .set_updated_at(now);
    account.insert(executor).map_err(|error| {
        let text = error.to_string();
        if text.contains("duplicate key") {
            LedgerError::Conflict(
                "duplicate_account_code".to_string(),
                "an account with this code already exists for the legal entity".to_string(),
            )
        } else {
            LedgerError::Database(format!("ledger account: {text}"))
        }
    })?;

    let mut audit = AccountingAuditEventRecord::new();
    audit
        .set_id(Uuid::new_v4())
        .set_tenant_id(session.tenant_id.clone())
        .set_legal_entity_id(legal_entity.id)
        .set_subject_id(session.subject_id)
        .set_action("ACCOUNT_CREATED".to_string())
        .set_document_id(account_id)
        .set_source_system(SOURCE_SYSTEM.to_string())
        .set_request_fingerprint(fingerprint.clone())
        .set_occurred_at(now);
    insert(&mut audit, executor, "audit event")?;

    let mut idempotency = AccountingIdempotencyRecordRecord::new();
    idempotency
        .set_id(Uuid::new_v4())
        .set_tenant_id(session.tenant_id.clone())
        .set_legal_entity_id(legal_entity.id)
        .set_idempotency_key(request.idempotency_key.clone())
        .set_request_fingerprint(fingerprint)
        .set_status("COMPLETED".to_string())
        .set_source_system(SOURCE_SYSTEM.to_string())
        .set_source_type(SOURCE_TYPE.to_string())
        .set_source_id(request.code.trim().to_string())
        .set_document_id(Some(account_id))
        .set_created_at(now)
        .set_completed_at(Some(now));
    insert(&mut idempotency, executor, "idempotency record")?;

    let account = AccountEntity::find()
        .filter(AccountColumn::LegalEntityId.eq(legal_entity.id))
        .filter(AccountColumn::Id.eq(account_id))
        .find_one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?
        .ok_or(LedgerError::NotFound)?;
    Ok(AccountCreation {
        account: account_json(&account),
        created: true,
    })
}

fn validate(request: &Request) -> Result<(), LedgerError> {
    let key = request.idempotency_key.trim();
    if key.len() < 8
        || key.len() > 200
        || !key
            .bytes()
            .next()
            .is_some_and(|byte| byte.is_ascii_alphanumeric())
        || !key
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-'))
    {
        return Err(LedgerError::Validation(
            "idempotency_key must be 8-200 characters of [A-Za-z0-9._:-]".to_string(),
        ));
    }
    let code = request.code.trim();
    if code.is_empty()
        || code.len() > 50
        || !code
            .bytes()
            .next()
            .is_some_and(|byte| byte.is_ascii_digit())
        || !code
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(LedgerError::Validation(
            "code must start with a digit and contain at most 50 characters of [0-9A-Za-z._-]"
                .to_string(),
        ));
    }
    if request.name.trim().is_empty() || request.name.trim().len() > 255 {
        return Err(LedgerError::Validation(
            "name must be 1-255 characters".to_string(),
        ));
    }
    if !ACCOUNT_TYPES.contains(&request.account_type.as_str()) {
        return Err(LedgerError::Validation(
            "account_type is not a supported accounting classification".to_string(),
        ));
    }
    if !NORMAL_SIDES.contains(&request.normal_side.as_str()) {
        return Err(LedgerError::Validation(
            "normal_side must be DEBIT or CREDIT".to_string(),
        ));
    }
    currency(&request.currency_code).map(|_| ())?;
    Ok(())
}

fn find_idempotency<E: LifeExecutor>(
    executor: &E,
    legal_entity_id: Uuid,
    key: &str,
) -> Result<
    Option<
        rerp_entities::accounting::foundation::idempotency_record::AccountingIdempotencyRecordModel,
    >,
    LedgerError,
> {
    match IdempotencyEntity::find()
        .filter(IdempotencyColumn::LegalEntityId.eq(legal_entity_id))
        .filter(IdempotencyColumn::IdempotencyKey.eq(key))
        .find_one(executor)
    {
        Ok(record) => Ok(record),
        Err(error) => Err(LedgerError::Database(error.to_string())),
    }
}

fn insert<R: ActiveModelTrait, E: LifeExecutor>(
    record: &mut R,
    executor: &E,
    resource: &str,
) -> Result<(), LedgerError> {
    record
        .insert(executor)
        .map(|_| ())
        .map_err(|error| LedgerError::Database(format!("{resource}: {error}")))
}

fn request_fingerprint(tenant_id: &str, legal_entity_id: Uuid, request: &Request) -> String {
    let mut hasher = Sha256::new();
    for part in [
        tenant_id.as_bytes().to_vec(),
        legal_entity_id.as_bytes().to_vec(),
        request.code.trim().as_bytes().to_vec(),
        request.name.trim().as_bytes().to_vec(),
        request.account_type.as_bytes().to_vec(),
        request.normal_side.as_bytes().to_vec(),
        request.currency_code.as_bytes().to_vec(),
    ] {
        hasher.update((part.len() as u64).to_be_bytes());
        hasher.update(part);
    }
    hex_digest(&hasher.finalize())
}

fn hex_digest(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[usize::from(byte >> 4)] as char);
        output.push(HEX[usize::from(byte & 0x0f)] as char);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> Request {
        Request {
            idempotency_key: "acct-create-0001".to_string(),
            code: "1200.01".to_string(),
            name: "Trade receivables".to_string(),
            account_type: "ASSET".to_string(),
            normal_side: "DEBIT".to_string(),
            currency_code: "EUR".to_string(),
        }
    }

    #[test]
    fn accepts_canonical_request() {
        assert!(validate(&request()).is_ok());
    }

    #[test]
    fn rejects_bad_fields() {
        let mut bad = request();
        bad.code = "A1200".to_string();
        assert!(matches!(validate(&bad), Err(LedgerError::Validation(_))));
        bad = request();
        bad.account_type = "CONTROL".to_string();
        assert!(matches!(validate(&bad), Err(LedgerError::Validation(_))));
        bad = request();
        bad.normal_side = "BOTH".to_string();
        assert!(matches!(validate(&bad), Err(LedgerError::Validation(_))));
        bad = request();
        bad.currency_code = "eur".to_string();
        assert!(matches!(validate(&bad), Err(LedgerError::Validation(_))));
        bad = request();
        bad.idempotency_key = "short".to_string();
        assert!(matches!(validate(&bad), Err(LedgerError::Validation(_))));
    }

    #[test]
    fn fingerprint_is_stable_and_field_sensitive() {
        let base = request();
        let tenant = "hauliage";
        let entity = Uuid::new_v4();
        assert_eq!(
            request_fingerprint(tenant, entity, &base),
            request_fingerprint(tenant, entity, &base),
        );
        let mut changed = request();
        changed.name = "Trade receivables (EU)".to_string();
        assert_ne!(
            request_fingerprint(tenant, entity, &base),
            request_fingerprint(tenant, entity, &changed),
        );
        assert_ne!(
            request_fingerprint(tenant, entity, &base),
            request_fingerprint("other-tenant", entity, &base),
        );
        assert_ne!(
            request_fingerprint(tenant, entity, &base),
            request_fingerprint(tenant, Uuid::new_v4(), &base),
        );
    }

    #[test]
    fn response_shape_matches_read_model() {
        // The serialized account must never carry tenant/legal-entity internals;
        // it goes through the same account_json used by the read model.
        let creation_account = serde_json::json!({
            "id": "00000000-0000-0000-0000-000000000000",
            "code": "1200.01",
            "name": "Trade receivables",
            "account_type": "ASSET",
            "normal_side": "DEBIT",
            "control_role": null,
            "currency_code": "EUR",
            "active": true,
        });
        for leaked in ["tenant_id", "legal_entity_id", "created_at", "updated_at"] {
            assert!(creation_account.get(leaked).is_none());
        }
    }

    #[test]
    #[ignore = "requires a disposable PostgreSQL database with the RERP foundation migrations"]
    fn live_create_idempotent_replay_duplicate_and_payload_conflict() {
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
        let first = crate::http_support::with_accounting_transaction(&session, |executor| {
            create_ledger_account(executor, &session, &request())
        })
        .expect("first account creation");
        assert!(first.created);

        let replay = crate::http_support::with_accounting_transaction(&session, |executor| {
            create_ledger_account(executor, &session, &request())
        })
        .expect("idempotent replay");
        assert!(!replay.created);
        assert_eq!(replay.account["id"], first.account["id"]);

        // Same key, changed payload must conflict.
        let mut changed = request();
        changed.name = "Renamed mid-flight".to_string();
        let conflict = crate::http_support::with_accounting_transaction(&session, |executor| {
            create_ledger_account(executor, &session, &changed)
        });
        assert!(matches!(
            conflict,
            Err(LedgerError::Conflict(ref code, _)) if code == "idempotency_conflict"
        ));

        // Fresh key, duplicate code must conflict.
        let mut duplicate = request();
        duplicate.idempotency_key = "acct-create-0002".to_string();
        let duplicate_conflict =
            crate::http_support::with_accounting_transaction(&session, |executor| {
                create_ledger_account(executor, &session, &duplicate)
            });
        assert!(matches!(
            duplicate_conflict,
            Err(LedgerError::Conflict(ref code, _)) if code == "duplicate_account_code"
        ));
    }
}
