//! Opening balances for a fiscal year (Epic D4a).
//!
//! The opening balances journal entry must be the FIRST posting in the
//! period covering January 1 of the fiscal year and can be set only once
//! per period. Numbering, persistence and audit conventions are shared
//! with the manual journal module; the retained-earnings close for year
//! transition is a separate operation and is deliberately NOT implied by
//! this endpoint.

use chrono::{NaiveDate, Utc};
use lifeguard::{ColumnTrait, LifeExecutor, LifeModelTrait, SessionContext};
use rust_decimal::Decimal;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use rerp_accounting_general_ledger_gen::handlers::set_opening_balances::Request;
use rerp_entities::accounting::foundation::account::{
    Column as AccountColumn, Entity as AccountEntity,
};
use rerp_entities::accounting::foundation::posted_document::{
    Column as DocumentColumn, Entity as DocumentEntity,
};

use crate::accounts::{find_idempotency, hex_digest, SOURCE_SYSTEM};
use crate::journals::{
    allocate_identifiers, check_totals_within_numeric_range, lock_and_revalidate_period,
    parse_amount, persist_posting, JournalDraftLine,
};
use crate::ledger::{currency, parse_uuid, resolve_legal_entity, LedgerError};

const DOCUMENT_TYPE_OPENING: &str = "OPENING_BALANCE";
const SOURCE_TYPE_OPENING: &str = "opening_balance";

pub struct OpeningChange {
    pub payload: Value,
    pub created: bool,
}

pub fn set_opening_balances<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    request: &Request,
) -> Result<OpeningChange, LedgerError> {
    let legal_entity = resolve_legal_entity(executor, session)?;
    let fingerprint = opening_fingerprint(session.tenant_id.as_str(), legal_entity.id, request);
    if let Some(record) = find_idempotency(executor, legal_entity.id, &request.idempotency_key)? {
        if record.request_fingerprint != fingerprint {
            return Err(LedgerError::Conflict(
                "idempotency_conflict".to_string(),
                "the idempotency key was already used with a different request body".to_string(),
            ));
        }
        let journal_id = record.journal_entry_id.ok_or(LedgerError::NotFound)?;
        let entry = rerp_entities::accounting::foundation::journal_entry::Entity::find()
            .filter(
                rerp_entities::accounting::foundation::journal_entry::Column::LegalEntityId
                    .eq(legal_entity.id),
            )
            .filter(rerp_entities::accounting::foundation::journal_entry::Column::Id.eq(journal_id))
            .find_one(executor)
            .map_err(|error| LedgerError::Database(error.to_string()))?
            .ok_or(LedgerError::NotFound)?;
        // The opening document's source_id is the journal id (persist_posting
        // convention), so the real document number is one lookup away.
        let document_number = DocumentEntity::find()
            .filter(DocumentColumn::LegalEntityId.eq(legal_entity.id))
            .filter(DocumentColumn::SourceType.eq(SOURCE_TYPE_OPENING))
            .filter(DocumentColumn::SourceId.eq(journal_id.to_string()))
            .find_one(executor)
            .map_err(|error| LedgerError::Database(error.to_string()))?
            .map(|document| document.document_number);
        return Ok(OpeningChange {
            payload: json!({
                "fiscal_year": request.fiscal_year,
                "fiscal_period_id": entry.fiscal_period_id.to_string(),
                "journal_entry_id": journal_id.to_string(),
                "entry_number": entry.entry_number,
                "document_number": document_number,
                "total_debit": format!("{:.6}", entry.total_debit),
                "total_credit": format!("{:.6}", entry.total_credit),
            }),
            created: false,
        });
    }

    let currency_code = currency(&request.currency_code)?;
    let year_start = NaiveDate::from_ymd_opt(request.fiscal_year, 1, 1).ok_or_else(|| {
        LedgerError::Validation("fiscal_year is outside the supported range".to_string())
    })?;
    // Eligibility (state, once-per-period, first-posting-only) is checked
    // under the per-period posting lock so a concurrent transition or
    // posting can never slip between check and persist.
    let period = lock_and_revalidate_period(executor, legal_entity.id, year_start)?;

    if already_set(executor, legal_entity.id, period.id)? {
        return Err(LedgerError::Conflict(
            "opening_balances_already_set".to_string(),
            "opening balances have already been set for this fiscal period".to_string(),
        ));
    }
    let postings = rerp_entities::accounting::foundation::journal_entry::Entity::find()
        .filter(
            rerp_entities::accounting::foundation::journal_entry::Column::FiscalPeriodId
                .eq(period.id),
        )
        .count()
        .one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;
    if postings > 0 {
        return Err(LedgerError::Conflict(
            "period_has_postings".to_string(),
            "opening balances must be the first posting in the fiscal period".to_string(),
        ));
    }

    let lines = validate_balances(executor, legal_entity.id, &currency_code, request)?;
    // Numbering and entry date follow the fiscal year (Jan 1), not the
    // period start: a period may legitimately begin in the prior calendar
    // year, and an opening entry must not leak into the prior year's
    // trial balance. Jan 1 is inside the period by construction.
    let ids = allocate_identifiers(
        executor,
        legal_entity.id,
        year_start,
        DOCUMENT_TYPE_OPENING,
        "OPB",
    )?;
    let now = Utc::now().naive_utc();
    persist_posting(
        executor,
        session,
        legal_entity.id,
        &ids,
        &lines,
        year_start,
        &currency_code,
        &period,
        now,
        &fingerprint,
        &request.idempotency_key,
        DOCUMENT_TYPE_OPENING,
        SOURCE_TYPE_OPENING,
        None,
        "OPENING_BALANCES_SET",
    )?;

    let total = lines
        .iter()
        .filter(|line| line.side == "DEBIT")
        .fold(Decimal::ZERO, |sum, line| sum + line.amount);
    Ok(OpeningChange {
        payload: json!({
            "fiscal_year": request.fiscal_year,
            "fiscal_period_id": period.id.to_string(),
            "journal_entry_id": ids.journal_id.to_string(),
            "entry_number": ids.entry_number,
            "document_number": ids.document_number,
            "total_debit": format!("{:.6}", total),
            "total_credit": format!("{:.6}", total),
        }),
        created: true,
    })
}

fn already_set<E: LifeExecutor>(
    executor: &E,
    legal_entity_id: Uuid,
    period_id: Uuid,
) -> Result<bool, LedgerError> {
    Ok(DocumentEntity::find()
        .filter(DocumentColumn::LegalEntityId.eq(legal_entity_id))
        .filter(DocumentColumn::FiscalPeriodId.eq(period_id))
        .filter(DocumentColumn::DocumentType.eq(DOCUMENT_TYPE_OPENING))
        .find_one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?
        .is_some())
}

fn validate_balances<E: LifeExecutor>(
    executor: &E,
    legal_entity_id: Uuid,
    currency_code: &str,
    request: &Request,
) -> Result<Vec<JournalDraftLine>, LedgerError> {
    let mut account_ids = Vec::with_capacity(request.balances.len());
    for balance in &request.balances {
        account_ids.push(parse_uuid(&balance.account_id, "account_id")?);
    }
    let accounts = AccountEntity::find()
        .filter(AccountColumn::LegalEntityId.eq(legal_entity_id))
        .filter(AccountColumn::Id.is_in(account_ids.iter().copied()))
        .all(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;
    let mut debit = Decimal::ZERO;
    let mut credit = Decimal::ZERO;
    let mut lines = Vec::with_capacity(request.balances.len());
    for (index, balance) in request.balances.iter().enumerate() {
        let account = accounts
            .iter()
            .find(|account| account.id == account_ids[index])
            .filter(|account| account.is_active)
            .ok_or_else(|| {
                LedgerError::Validation(format!(
                    "balance {} references an unknown or inactive account",
                    index + 1
                ))
            })?;
        if account.currency_code != currency_code {
            return Err(LedgerError::Validation(format!(
                "balance {} account currency differs from the entry currency",
                index + 1
            )));
        }
        let amount = parse_amount(&balance.amount, index + 1)?;
        match balance.side.as_str() {
            "DEBIT" => debit += amount,
            "CREDIT" => credit += amount,
            _ => {
                return Err(LedgerError::Validation(format!(
                    "balance {} side must be DEBIT or CREDIT",
                    index + 1
                )))
            }
        }
        lines.push(JournalDraftLine {
            account_id: account.id,
            side: balance.side.clone(),
            amount,
            description: balance
                .description
                .clone()
                .unwrap_or_else(|| "Opening balance".to_string()),
        });
    }
    if debit != credit {
        return Err(LedgerError::Validation(format!(
            "opening balances are unbalanced: debits {debit} != credits {credit}"
        )));
    }
    check_totals_within_numeric_range(debit, credit)?;
    Ok(lines)
}

fn opening_fingerprint(tenant_id: &str, legal_entity_id: Uuid, request: &Request) -> String {
    let mut parts: Vec<Vec<u8>> = vec![
        tenant_id.as_bytes().to_vec(),
        legal_entity_id.as_bytes().to_vec(),
        b"SET_OPENING_BALANCES".to_vec(),
        request.fiscal_year.to_string().into_bytes(),
        request.currency_code.as_bytes().to_vec(),
    ];
    for balance in &request.balances {
        let description = balance.description.as_deref().unwrap_or("");
        let joined = format!(
            "{}|{}|{}|{}",
            balance.account_id.trim().to_lowercase(),
            balance.side,
            balance.amount,
            description
        );
        parts.push(joined.into_bytes());
    }
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
    use rerp_accounting_general_ledger_gen::handlers::types::OpeningBalanceInput;

    fn balance(account: &str, side: &str, amount: &str) -> OpeningBalanceInput {
        OpeningBalanceInput {
            account_id: account.to_string(),
            side: side.to_string(),
            amount: amount.to_string(),
            description: None,
        }
    }

    fn request(balances: Vec<OpeningBalanceInput>) -> Request {
        request_with_year(2026, balances)
    }

    fn request_with_year(fiscal_year: i32, balances: Vec<OpeningBalanceInput>) -> Request {
        Request {
            idempotency_key: "open-0001".to_string(),
            fiscal_year,
            currency_code: "EUR".to_string(),
            balances,
        }
    }

    #[test]
    fn fingerprint_binds_year_currency_and_balance_set() {
        let entity = Uuid::new_v4();
        let cash = Uuid::new_v4().to_string();
        let equity = Uuid::new_v4().to_string();
        let base = request(vec![
            balance(&cash, "DEBIT", "1000.00"),
            balance(&equity, "CREDIT", "1000.00"),
        ]);
        let other_year = request_with_year(
            2027,
            vec![
                balance(&cash, "DEBIT", "1000.00"),
                balance(&equity, "CREDIT", "1000.00"),
            ],
        );
        let swapped = request(vec![
            balance(&equity, "CREDIT", "1000.00"),
            balance(&cash, "DEBIT", "1000.00"),
        ]);
        let other_amount = request(vec![
            balance(&cash, "DEBIT", "1000.01"),
            balance(&equity, "CREDIT", "999.99"),
        ]);
        let fingerprint = opening_fingerprint("hauliage", entity, &base);
        assert_eq!(fingerprint, opening_fingerprint("hauliage", entity, &base));
        assert_ne!(
            fingerprint,
            opening_fingerprint("hauliage", entity, &other_year)
        );
        assert_ne!(
            fingerprint,
            opening_fingerprint("hauliage", entity, &swapped)
        );
        assert_ne!(
            fingerprint,
            opening_fingerprint("hauliage", entity, &other_amount)
        );
    }

    #[test]
    #[ignore = "requires a disposable PostgreSQL database with the RERP foundation migrations"]
    fn live_opening_balances_set_replay_and_conflicts() {
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
        // Fixture: an OPEN period covering 2027-01-01 with no postings yet.
        let cash = Uuid::parse_str("c3000003-0003-4000-8000-000000000005").expect("cash");
        let equity = Uuid::parse_str("c3000003-0003-4000-8000-000000000006").expect("equity");
        let balances = request(vec![
            balance(&cash.to_string(), "DEBIT", "5000.00"),
            balance(&equity.to_string(), "CREDIT", "5000.00"),
        ]);
        let set = crate::http_support::with_accounting_transaction(&session, |executor| {
            set_opening_balances(executor, &session, &balances)
        })
        .expect("opening balances set");
        assert!(set.created);
        assert_eq!(set.payload["total_debit"], "5000.000000");

        let replay = crate::http_support::with_accounting_transaction(&session, |executor| {
            set_opening_balances(executor, &session, &balances)
        })
        .expect("idempotent replay");
        assert!(!replay.created);
        assert_eq!(
            replay.payload["journal_entry_id"],
            set.payload["journal_entry_id"]
        );

        // Fresh key, different balances: already-set conflict.
        let fresh = request_with_year(
            2027,
            vec![
                balance(&cash.to_string(), "DEBIT", "6000.00"),
                balance(&equity.to_string(), "CREDIT", "6000.00"),
            ],
        );
        let conflict = crate::http_support::with_accounting_transaction(&session, |executor| {
            set_opening_balances(executor, &session, &fresh)
        });
        assert!(matches!(
            conflict,
            Err(LedgerError::Conflict(ref code, _)) if code == "opening_balances_already_set"
        ));
    }
}
