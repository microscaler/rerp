//! Retained-earnings fiscal year close (Epic D4b).
//!
//! Posts the closing journal entry that moves the net result of every
//! REVENUE and EXPENSE account posted in a fiscal year's periods to a
//! designated EQUITY retained-earnings account. Closing entries
//! themselves are excluded from aggregation, a year closes once, and a
//! year without P&L activity cannot be closed. Shares the journals
//! machinery for numbering, persistence and audit.

use std::collections::BTreeMap;

use chrono::{NaiveDate, Utc};
use lifeguard::{ColumnTrait, LifeExecutor, LifeModelTrait, SessionContext};
use rust_decimal::Decimal;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use rerp_accounting_general_ledger_gen::handlers::close_fiscal_year::Request;
use rerp_entities::accounting::foundation::account::{
    Column as AccountColumn, Entity as AccountEntity,
};
use rerp_entities::accounting::foundation::fiscal_period::{
    Column as PeriodColumn, Entity as PeriodEntity,
};
use rerp_entities::accounting::foundation::journal_entry::{
    Column as JournalColumn, Entity as JournalEntity,
};
use rerp_entities::accounting::foundation::journal_line::{
    Column as LineColumn, Entity as LineEntity,
};
use rerp_entities::accounting::foundation::posted_document::{
    Column as DocumentColumn, Entity as DocumentEntity,
};

use crate::accounts::{find_idempotency, hex_digest};
use crate::journals::{
    allocate_identifiers, lock_and_revalidate_period, persist_posting, JournalDraftLine,
};
use crate::ledger::{parse_uuid, resolve_legal_entity, LedgerError};

const DOCUMENT_TYPE_CLOSE: &str = "YEAR_CLOSE";
const SOURCE_TYPE_CLOSE: &str = "year_close";
const MAX_PL_ACCOUNTS: usize = 500;

pub struct YearCloseChange {
    pub payload: Value,
    pub created: bool,
}

pub fn close_fiscal_year<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    request: &Request,
) -> Result<YearCloseChange, LedgerError> {
    let legal_entity = resolve_legal_entity(executor, session)?;
    let fingerprint = close_fingerprint(session.tenant_id.as_str(), legal_entity.id, request);
    if let Some(record) = find_idempotency(executor, legal_entity.id, &request.idempotency_key)? {
        if record.request_fingerprint != fingerprint {
            return Err(LedgerError::Conflict(
                "idempotency_conflict".to_string(),
                "the idempotency key was already used with a different request body".to_string(),
            ));
        }
        return replay_payload(executor, legal_entity.id, record.journal_entry_id, request);
    }

    let re_account_id = parse_uuid(
        &request.retained_earnings_account_id,
        "retained_earnings_account_id",
    )?;
    let re_account = AccountEntity::find()
        .filter(AccountColumn::LegalEntityId.eq(legal_entity.id))
        .filter(AccountColumn::Id.eq(re_account_id))
        .find_one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?
        .filter(|account| account.is_active && account.account_type == "EQUITY")
        .ok_or_else(|| {
            LedgerError::Validation(
                "retained_earnings_account_id must reference an active EQUITY account".to_string(),
            )
        })?;

    let year_start = NaiveDate::from_ymd_opt(request.fiscal_year, 1, 1).ok_or_else(|| {
        LedgerError::Validation("fiscal_year is outside the supported range".to_string())
    })?;
    let year_end = NaiveDate::from_ymd_opt(request.fiscal_year, 12, 31).ok_or_else(|| {
        LedgerError::Validation("fiscal_year is outside the supported range".to_string())
    })?;
    let periods = PeriodEntity::find()
        .filter(PeriodColumn::LegalEntityId.eq(legal_entity.id))
        .filter(PeriodColumn::StartDate.between(year_start, year_end))
        .all(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;
    if periods.is_empty() {
        return Err(LedgerError::Conflict(
            "no_periods_for_year".to_string(),
            "no fiscal periods start within the requested fiscal year".to_string(),
        ));
    }
    let period_ids: Vec<Uuid> = periods.iter().map(|period| period.id).collect();

    // The close entry itself must be postable: the period covering
    // December 31 must be open for posting (OPEN or REOPENED), checked
    // under the per-period posting lock together with the once-per-year
    // eligibility checks below.
    let period = lock_and_revalidate_period(executor, legal_entity.id, year_end)?;

    if DocumentEntity::find()
        .filter(DocumentColumn::LegalEntityId.eq(legal_entity.id))
        .filter(DocumentColumn::DocumentType.eq(DOCUMENT_TYPE_CLOSE))
        .filter(DocumentColumn::FiscalPeriodId.is_in(period_ids.iter().copied()))
        .find_one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?
        .is_some()
    {
        return Err(LedgerError::Conflict(
            "year_already_closed".to_string(),
            "this fiscal year has already been closed".to_string(),
        ));
    }

    let entries = JournalEntity::find()
        .filter(JournalColumn::LegalEntityId.eq(legal_entity.id))
        .filter(JournalColumn::FiscalPeriodId.is_in(period_ids.iter().copied()))
        .filter(JournalColumn::EntryDate.between(year_start, year_end))
        .all(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;
    let entry_ids: Vec<Uuid> = entries.iter().map(|entry| entry.id).collect();
    let lines = if entry_ids.is_empty() {
        Vec::new()
    } else {
        LineEntity::find()
            .filter(LineColumn::LegalEntityId.eq(legal_entity.id))
            .filter(LineColumn::JournalEntryId.is_in(entry_ids.iter().copied()))
            .all(executor)
            .map_err(|error| LedgerError::Database(error.to_string()))?
    };

    let accounts = AccountEntity::find()
        .filter(AccountColumn::LegalEntityId.eq(legal_entity.id))
        .all(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;
    let mut nets: BTreeMap<Uuid, Decimal> = BTreeMap::new();
    for line in &lines {
        let Some(account) = accounts
            .iter()
            .find(|account| account.id == line.account_id)
        else {
            continue;
        };
        if account.account_type != "REVENUE" && account.account_type != "EXPENSE" {
            continue;
        }
        if account.currency_code != re_account.currency_code {
            return Err(LedgerError::Validation(format!(
                "account {} has currency {} which differs from the retained earnings currency {}",
                account.code, account.currency_code, re_account.currency_code
            )));
        }
        let signed = if line.side == "CREDIT" {
            line.amount
        } else {
            -line.amount
        };
        *nets.entry(line.account_id).or_insert(Decimal::ZERO) += signed;
    }
    nets.retain(|_, net| *net != Decimal::ZERO);
    if nets.is_empty() {
        return Err(LedgerError::Conflict(
            "nothing_to_close".to_string(),
            "the fiscal year has no profit and loss postings to close".to_string(),
        ));
    }
    if nets.len() > MAX_PL_ACCOUNTS {
        return Err(LedgerError::Conflict(
            "close_scope_too_large".to_string(),
            format!(
                "the year touches {} profit and loss accounts, above the {} limit",
                nets.len(),
                MAX_PL_ACCOUNTS
            ),
        ));
    }

    let mut drafts: Vec<JournalDraftLine> = Vec::with_capacity(nets.len() + 1);
    let mut net_income = Decimal::ZERO;
    for (account_id, net) in &nets {
        let account = accounts
            .iter()
            .find(|account| account.id == *account_id)
            .expect("account present after aggregation");
        let (side, amount) = if *net > Decimal::ZERO {
            ("DEBIT".to_string(), *net)
        } else {
            ("CREDIT".to_string(), -*net)
        };
        net_income += net;
        drafts.push(JournalDraftLine {
            account_id: *account_id,
            side,
            amount,
            description: format!(
                "Fiscal year {} close of {}",
                request.fiscal_year, account.code
            ),
        });
    }
    if net_income != Decimal::ZERO {
        drafts.push(JournalDraftLine {
            account_id: re_account.id,
            side: if net_income > Decimal::ZERO {
                "CREDIT".to_string()
            } else {
                "DEBIT".to_string()
            },
            amount: if net_income > Decimal::ZERO {
                net_income
            } else {
                -net_income
            },
            description: format!("Retained earnings for fiscal year {}", request.fiscal_year),
        });
    }

    let ids = allocate_identifiers(
        executor,
        legal_entity.id,
        year_end,
        DOCUMENT_TYPE_CLOSE,
        "YCL",
    )?;
    let now = Utc::now().naive_utc();
    persist_posting(
        executor,
        session,
        legal_entity.id,
        &ids,
        &drafts,
        year_end,
        &re_account.currency_code,
        &period,
        now,
        &fingerprint,
        &request.idempotency_key,
        DOCUMENT_TYPE_CLOSE,
        SOURCE_TYPE_CLOSE,
        None,
        "FISCAL_YEAR_CLOSED",
    )?;

    Ok(YearCloseChange {
        payload: close_payload(
            request,
            &ids.journal_id,
            &ids.entry_number,
            Some(&ids.document_number),
            net_income,
            drafts.len(),
        ),
        created: true,
    })
}

fn replay_payload<E: LifeExecutor>(
    executor: &E,
    legal_entity_id: Uuid,
    journal_entry_id: Option<Uuid>,
    request: &Request,
) -> Result<YearCloseChange, LedgerError> {
    let journal_entry_id = journal_entry_id.ok_or(LedgerError::NotFound)?;
    let entry = JournalEntity::find()
        .filter(JournalColumn::LegalEntityId.eq(legal_entity_id))
        .filter(JournalColumn::Id.eq(journal_entry_id))
        .find_one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?
        .ok_or(LedgerError::NotFound)?;
    let document_number = DocumentEntity::find()
        .filter(DocumentColumn::LegalEntityId.eq(legal_entity_id))
        .filter(DocumentColumn::SourceType.eq(SOURCE_TYPE_CLOSE))
        .filter(DocumentColumn::SourceId.eq(journal_entry_id.to_string()))
        .find_one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?
        .map(|document| document.document_number);
    let re_account_id = parse_uuid(
        &request.retained_earnings_account_id,
        "retained_earnings_account_id",
    )?;
    let re_line = LineEntity::find()
        .filter(LineColumn::LegalEntityId.eq(legal_entity_id))
        .filter(LineColumn::JournalEntryId.eq(journal_entry_id))
        .filter(LineColumn::AccountId.eq(re_account_id))
        .find_one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;
    let line_count = LineEntity::find()
        .filter(LineColumn::LegalEntityId.eq(legal_entity_id))
        .filter(LineColumn::JournalEntryId.eq(journal_entry_id))
        .count()
        .one(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;
    let net_income = match re_line {
        Some(line) if line.side == "CREDIT" => line.amount,
        Some(line) => -line.amount,
        None => Decimal::ZERO,
    };
    Ok(YearCloseChange {
        payload: close_payload(
            request,
            &journal_entry_id,
            &entry.entry_number,
            document_number.as_deref(),
            net_income,
            line_count as usize,
        ),
        created: false,
    })
}

fn close_payload(
    request: &Request,
    journal_entry_id: &Uuid,
    entry_number: &str,
    document_number: Option<&str>,
    net_income: Decimal,
    line_count: usize,
) -> Value {
    json!({
        "fiscal_year": request.fiscal_year,
        "journal_entry_id": journal_entry_id.to_string(),
        "entry_number": entry_number,
        "document_number": document_number,
        "retained_earnings_account_id": request.retained_earnings_account_id,
        "net_income": format!("{:.6}", net_income),
        "line_count": line_count,
    })
}

fn close_fingerprint(tenant_id: &str, legal_entity_id: Uuid, request: &Request) -> String {
    let mut hasher = Sha256::new();
    for part in [
        tenant_id.as_bytes().to_vec(),
        legal_entity_id.as_bytes().to_vec(),
        b"CLOSE_FISCAL_YEAR".to_vec(),
        request.fiscal_year.to_string().into_bytes(),
        request
            .retained_earnings_account_id
            .trim()
            .to_lowercase()
            .into_bytes(),
    ] {
        hasher.update((part.len() as u64).to_be_bytes());
        hasher.update(part);
    }
    hex_digest(&hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(year: i32, re: &str) -> Request {
        Request {
            idempotency_key: "close-0001".to_string(),
            fiscal_year: year,
            retained_earnings_account_id: re.to_string(),
        }
    }

    #[test]
    fn fingerprint_binds_year_and_retained_earnings_account() {
        let entity = Uuid::new_v4();
        let re = Uuid::new_v4().to_string();
        let other_re = Uuid::new_v4().to_string();
        let base = request(2026, &re);
        assert_eq!(
            close_fingerprint("hauliage", entity, &base),
            close_fingerprint("hauliage", entity, &request(2026, &re))
        );
        assert_ne!(
            close_fingerprint("hauliage", entity, &base),
            close_fingerprint("hauliage", entity, &request(2025, &re))
        );
        assert_ne!(
            close_fingerprint("hauliage", entity, &base),
            close_fingerprint("hauliage", entity, &request(2026, &other_re))
        );
        assert_ne!(
            close_fingerprint("hauliage", entity, &base),
            close_fingerprint("hauliage", Uuid::new_v4(), &base)
        );
    }

    #[test]
    #[ignore = "requires a disposable PostgreSQL database with the RERP foundation migrations"]
    fn live_year_close_posts_balanced_re_entry_and_replays() {
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
        // Fixture: an open 2027 period, revenue and expense postings in
        // 2027, and an active EQUITY account for retained earnings.
        let equity = Uuid::parse_str("c3000003-0003-4000-8000-000000000007").expect("equity UUID");
        let close = request(2027, &equity.to_string());
        let posted = crate::http_support::with_accounting_transaction(&session, |executor| {
            close_fiscal_year(executor, &session, &close)
        })
        .expect("year close posted");
        assert!(posted.created);
        assert_eq!(
            posted.payload["document_number"]
                .as_str()
                .expect("document number")
                .split('-')
                .next()
                .expect("prefix"),
            "YCL"
        );

        let replay = crate::http_support::with_accounting_transaction(&session, |executor| {
            close_fiscal_year(executor, &session, &close)
        })
        .expect("idempotent replay");
        assert!(!replay.created);
        assert_eq!(
            replay.payload["journal_entry_id"],
            posted.payload["journal_entry_id"]
        );

        // Fresh key: a year cannot close twice.
        let fresh = Request {
            idempotency_key: "close-0002".to_string(),
            fiscal_year: 2027,
            retained_earnings_account_id: equity.to_string(),
        };
        let conflict = crate::http_support::with_accounting_transaction(&session, |executor| {
            close_fiscal_year(executor, &session, &fresh)
        });
        assert!(matches!(
            conflict,
            Err(LedgerError::Conflict(ref code, _)) if code == "year_already_closed"
        ));
    }
}
