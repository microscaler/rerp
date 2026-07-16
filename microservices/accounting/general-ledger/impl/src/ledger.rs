//! Tenant-safe read model over the canonical accounting foundation.

use std::collections::{BTreeMap, HashMap};

use chrono::NaiveDate;
use lifeguard::{ColumnTrait, LifeExecutor, LifeModelTrait, Order, SessionContext};
use rerp_entities::accounting::foundation::account::{
    AccountingAccountModel, Column as AccountColumn, Entity as AccountEntity,
};
use rerp_entities::accounting::foundation::fiscal_period::{
    Column as PeriodColumn, Entity as PeriodEntity,
};
use rerp_entities::accounting::foundation::journal_entry::{
    AccountingJournalEntryModel, Column as JournalColumn, Entity as JournalEntity,
};
use rerp_entities::accounting::foundation::journal_line::{
    AccountingJournalLineModel, Column as JournalLineColumn, Entity as JournalLineEntity,
};
use rerp_entities::accounting::foundation::legal_entity::{
    AccountingLegalEntityModel, Column as LegalEntityColumn, Entity as LegalEntityEntity,
};
use rust_decimal::Decimal;
use serde_json::{json, Value};
use uuid::Uuid;

const DEFAULT_LIMIT: i32 = 50;
const MAX_LIMIT: i32 = 100;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LedgerError {
    Validation(String),
    Policy(String),
    NotFound,
    Database(String),
}

impl std::fmt::Display for LedgerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation(message) => write!(formatter, "validation failed: {message}"),
            Self::Policy(message) => {
                write!(formatter, "accounting policy rejected query: {message}")
            }
            Self::NotFound => formatter.write_str("ledger resource not found"),
            Self::Database(message) => write!(formatter, "database operation failed: {message}"),
        }
    }
}

impl std::error::Error for LedgerError {}

pub fn parse_uuid(value: &str, field: &str) -> Result<Uuid, LedgerError> {
    Uuid::parse_str(value.trim())
        .map_err(|_| LedgerError::Validation(format!("{field} must be a UUID")))
}

fn parse_date(value: &str, field: &str) -> Result<NaiveDate, LedgerError> {
    NaiveDate::parse_from_str(value.trim(), "%Y-%m-%d")
        .map_err(|_| LedgerError::Validation(format!("{field} must be an ISO 8601 date")))
}

fn currency(value: &str) -> Result<String, LedgerError> {
    let value = value.trim();
    if value.len() != 3 || !value.bytes().all(|byte| byte.is_ascii_uppercase()) {
        return Err(LedgerError::Validation(
            "currency_code must contain three uppercase ASCII letters".to_string(),
        ));
    }
    Ok(value.to_string())
}

fn limit(value: Option<i32>) -> Result<i32, LedgerError> {
    let value = value.unwrap_or(DEFAULT_LIMIT);
    if !(1..=MAX_LIMIT).contains(&value) {
        return Err(LedgerError::Validation(format!(
            "limit must be between 1 and {MAX_LIMIT}"
        )));
    }
    Ok(value)
}

fn resolve_legal_entity<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
) -> Result<AccountingLegalEntityModel, LedgerError> {
    match LegalEntityEntity::find()
        .filter(LegalEntityColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(LegalEntityColumn::OrganizationId.eq(session.organization_id))
        .filter(LegalEntityColumn::IsActive.eq(true))
        .find_one(executor)
    {
        Ok(Some(value)) => Ok(value),
        Ok(None) => Err(LedgerError::Policy(
            "an active legal entity is not configured for this organization".to_string(),
        )),
        Err(error) => Err(LedgerError::Database(error.to_string())),
    }
}

pub fn list_accounts<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    account_type: Option<&str>,
    currency_code: Option<&str>,
    active: Option<bool>,
    requested_limit: Option<i32>,
) -> Result<Value, LedgerError> {
    let legal_entity = resolve_legal_entity(executor, session)?;
    let account_type = account_type
        .map(str::trim)
        .map(str::to_uppercase)
        .map(|value| match value.as_str() {
            "ASSET" | "LIABILITY" | "EQUITY" | "REVENUE" | "EXPENSE" => Ok(value),
            _ => Err(LedgerError::Validation(
                "account_type is not a supported accounting classification".to_string(),
            )),
        })
        .transpose()?;
    let currency_code = currency_code.map(currency).transpose()?;
    let active = active.unwrap_or(true);
    let limit = limit(requested_limit)?;

    let mut query = AccountEntity::find()
        .filter(AccountColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(AccountColumn::LegalEntityId.eq(legal_entity.id))
        .filter(AccountColumn::IsActive.eq(active));
    if let Some(value) = account_type {
        query = query.filter(AccountColumn::AccountType.eq(value));
    }
    if let Some(value) = currency_code {
        query = query.filter(AccountColumn::CurrencyCode.eq(value));
    }
    let accounts = query
        .order_by(AccountColumn::Code, Order::Asc)
        .limit(limit as u64)
        .all(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;

    Ok(json!({
        "items": accounts.iter().map(account_json).collect::<Vec<_>>(),
        "limit": limit,
    }))
}

pub fn list_fiscal_periods<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    state: Option<&str>,
    from_date: Option<&str>,
    to_date: Option<&str>,
    requested_limit: Option<i32>,
) -> Result<Value, LedgerError> {
    let legal_entity = resolve_legal_entity(executor, session)?;
    let state = state
        .map(str::trim)
        .map(str::to_uppercase)
        .map(|value| match value.as_str() {
            "OPEN" | "CLOSED" | "HARD_LOCKED" => Ok(value),
            _ => Err(LedgerError::Validation(
                "state must be OPEN, CLOSED, or HARD_LOCKED".to_string(),
            )),
        })
        .transpose()?;
    let from_date = from_date
        .map(|value| parse_date(value, "from_date"))
        .transpose()?;
    let to_date = to_date
        .map(|value| parse_date(value, "to_date"))
        .transpose()?;
    if matches!((from_date, to_date), (Some(from), Some(to)) if from > to) {
        return Err(LedgerError::Validation(
            "from_date must not be after to_date".to_string(),
        ));
    }
    let limit = limit(requested_limit)?;

    let mut query = PeriodEntity::find()
        .filter(PeriodColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(PeriodColumn::LegalEntityId.eq(legal_entity.id));
    if let Some(value) = state {
        query = query.filter(PeriodColumn::State.eq(value));
    }
    if let Some(value) = from_date {
        query = query.filter(PeriodColumn::EndDate.gte(value));
    }
    if let Some(value) = to_date {
        query = query.filter(PeriodColumn::StartDate.lte(value));
    }
    let periods = query
        .order_by(PeriodColumn::StartDate, Order::Desc)
        .limit(limit as u64)
        .all(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;

    Ok(json!({
        "items": periods.into_iter().map(|period| json!({
            "id": period.id,
            "name": period.name,
            "start_date": period.start_date,
            "end_date": period.end_date,
            "state": period.state,
            "closed_at": period.closed_at.map(|value| value.and_utc().to_rfc3339()),
        })).collect::<Vec<_>>(),
        "limit": limit,
    }))
}

pub fn get_journal_entry<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    journal_id: Uuid,
) -> Result<Value, LedgerError> {
    let legal_entity = resolve_legal_entity(executor, session)?;
    let entry = match JournalEntity::find()
        .filter(JournalColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(JournalColumn::LegalEntityId.eq(legal_entity.id))
        .filter(JournalColumn::Id.eq(journal_id))
        .find_one(executor)
    {
        Ok(Some(value)) => value,
        Ok(None) => return Err(LedgerError::NotFound),
        Err(error) => return Err(LedgerError::Database(error.to_string())),
    };
    let lines = JournalLineEntity::find()
        .filter(JournalLineColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(JournalLineColumn::LegalEntityId.eq(legal_entity.id))
        .filter(JournalLineColumn::JournalEntryId.eq(entry.id))
        .order_by(JournalLineColumn::LineNumber, Order::Asc)
        .all(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;
    validate_journal(&entry, &lines)?;

    Ok(json!({
        "id": entry.id,
        "fiscal_period_id": entry.fiscal_period_id,
        "entry_number": entry.entry_number,
        "entry_date": entry.entry_date,
        "source_document_id": entry.source_document_id,
        "currency_code": entry.currency_code,
        "total_debit": decimal(entry.total_debit),
        "total_credit": decimal(entry.total_credit),
        "posted_at": entry.posted_at.and_utc().to_rfc3339(),
        "lines": lines.iter().map(journal_line_json).collect::<Vec<_>>(),
    }))
}

pub fn get_trial_balance<E: LifeExecutor>(
    executor: &E,
    session: &SessionContext,
    as_of_date: &str,
    currency_code: &str,
    include_zero_balance: Option<bool>,
) -> Result<Value, LedgerError> {
    let legal_entity = resolve_legal_entity(executor, session)?;
    let as_of_date = parse_date(as_of_date, "as_of_date")?;
    let currency_code = currency(currency_code)?;
    let include_zero_balance = include_zero_balance.unwrap_or(false);

    let entries = JournalEntity::find()
        .filter(JournalColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(JournalColumn::LegalEntityId.eq(legal_entity.id))
        .filter(JournalColumn::EntryDate.lte(as_of_date))
        .filter(JournalColumn::CurrencyCode.eq(currency_code.clone()))
        .all(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;
    let entry_ids = entries.iter().map(|entry| entry.id).collect::<Vec<_>>();
    let lines = if entry_ids.is_empty() {
        Vec::new()
    } else {
        JournalLineEntity::find()
            .filter(JournalLineColumn::TenantId.eq(session.tenant_id.clone()))
            .filter(JournalLineColumn::LegalEntityId.eq(legal_entity.id))
            .filter(JournalLineColumn::JournalEntryId.is_in(entry_ids))
            .all(executor)
            .map_err(|error| LedgerError::Database(error.to_string()))?
    };
    let accounts = AccountEntity::find()
        .filter(AccountColumn::TenantId.eq(session.tenant_id.clone()))
        .filter(AccountColumn::LegalEntityId.eq(legal_entity.id))
        .filter(AccountColumn::CurrencyCode.eq(currency_code.clone()))
        .order_by(AccountColumn::Code, Order::Asc)
        .all(executor)
        .map_err(|error| LedgerError::Database(error.to_string()))?;
    validate_trial_balance_source(&entries, &lines)?;
    let trial_balance = derive_trial_balance(&accounts, &lines, include_zero_balance)?;

    Ok(json!({
        "as_of_date": as_of_date,
        "currency_code": currency_code,
        "total_debit": decimal(trial_balance.total_debit),
        "total_credit": decimal(trial_balance.total_credit),
        "difference": decimal(trial_balance.total_debit - trial_balance.total_credit),
        "balanced": trial_balance.total_debit == trial_balance.total_credit,
        "lines": trial_balance.lines,
    }))
}

fn account_json(account: &AccountingAccountModel) -> Value {
    json!({
        "id": account.id,
        "code": account.code,
        "name": account.name,
        "account_type": account.account_type,
        "normal_side": account.normal_side,
        "control_role": account.control_role,
        "currency_code": account.currency_code,
        "active": account.is_active,
    })
}

fn journal_line_json(line: &AccountingJournalLineModel) -> Value {
    json!({
        "line_number": line.line_number,
        "account_id": line.account_id,
        "side": line.side,
        "amount": decimal(line.amount),
        "description": line.description,
    })
}

fn validate_journal(
    entry: &AccountingJournalEntryModel,
    lines: &[AccountingJournalLineModel],
) -> Result<(), LedgerError> {
    let totals = line_totals(lines)?;
    if totals.0 != entry.total_debit || totals.1 != entry.total_credit || totals.0 != totals.1 {
        return Err(LedgerError::Database(format!(
            "journal {} failed immutable balance checks",
            entry.id
        )));
    }
    Ok(())
}

fn validate_trial_balance_source(
    entries: &[AccountingJournalEntryModel],
    lines: &[AccountingJournalLineModel],
) -> Result<(), LedgerError> {
    let mut totals = entries
        .iter()
        .map(|entry| (entry.id, (Decimal::ZERO, Decimal::ZERO, 0usize)))
        .collect::<HashMap<_, _>>();
    for line in lines {
        if line.amount <= Decimal::ZERO {
            return Err(LedgerError::Database(format!(
                "journal line {} has a non-positive amount",
                line.id
            )));
        }
        let total = totals.get_mut(&line.journal_entry_id).ok_or_else(|| {
            LedgerError::Database(format!(
                "journal line {} references an unselected journal",
                line.id
            ))
        })?;
        match line.side.as_str() {
            "DEBIT" => total.0 += line.amount,
            "CREDIT" => total.1 += line.amount,
            other => {
                return Err(LedgerError::Database(format!(
                    "journal line {} has invalid side {other}",
                    line.id
                )))
            }
        }
        total.2 += 1;
    }
    for entry in entries {
        let (debit, credit, line_count) = totals[&entry.id];
        if line_count < 2
            || debit != entry.total_debit
            || credit != entry.total_credit
            || debit != credit
        {
            return Err(LedgerError::Database(format!(
                "journal {} failed immutable balance checks",
                entry.id
            )));
        }
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct DerivedTrialBalance {
    total_debit: Decimal,
    total_credit: Decimal,
    lines: Vec<Value>,
}

fn derive_trial_balance(
    accounts: &[AccountingAccountModel],
    lines: &[AccountingJournalLineModel],
    include_zero_balance: bool,
) -> Result<DerivedTrialBalance, LedgerError> {
    let accounts_by_id = accounts
        .iter()
        .map(|account| (account.id, account))
        .collect::<HashMap<_, _>>();
    let mut balances = BTreeMap::<Uuid, (Decimal, Decimal)>::new();
    let (total_debit, total_credit) = line_totals(lines)?;
    for line in lines {
        if !accounts_by_id.contains_key(&line.account_id) {
            return Err(LedgerError::Database(format!(
                "journal line references account {} outside the requested currency book",
                line.account_id
            )));
        }
        let balance = balances
            .entry(line.account_id)
            .or_insert((Decimal::ZERO, Decimal::ZERO));
        match line.side.as_str() {
            "DEBIT" => balance.0 += line.amount,
            "CREDIT" => balance.1 += line.amount,
            _ => unreachable!("line_totals validates sides"),
        }
    }

    let mut output = Vec::new();
    for account in accounts {
        let (debit, credit) = balances
            .get(&account.id)
            .copied()
            .unwrap_or((Decimal::ZERO, Decimal::ZERO));
        let balance = match account.normal_side.as_str() {
            "DEBIT" => debit - credit,
            "CREDIT" => credit - debit,
            other => {
                return Err(LedgerError::Database(format!(
                    "account {} has invalid normal side {other}",
                    account.id
                )))
            }
        };
        if include_zero_balance || debit != Decimal::ZERO || credit != Decimal::ZERO {
            output.push(json!({
                "account_id": account.id,
                "account_code": account.code,
                "account_name": account.name,
                "account_type": account.account_type,
                "normal_side": account.normal_side,
                "debit": decimal(debit),
                "credit": decimal(credit),
                "balance": decimal(balance),
            }));
        }
    }
    Ok(DerivedTrialBalance {
        total_debit,
        total_credit,
        lines: output,
    })
}

fn line_totals(lines: &[AccountingJournalLineModel]) -> Result<(Decimal, Decimal), LedgerError> {
    let mut debit = Decimal::ZERO;
    let mut credit = Decimal::ZERO;
    for line in lines {
        if line.amount <= Decimal::ZERO {
            return Err(LedgerError::Database(format!(
                "journal line {} has a non-positive amount",
                line.id
            )));
        }
        match line.side.as_str() {
            "DEBIT" => debit += line.amount,
            "CREDIT" => credit += line.amount,
            other => {
                return Err(LedgerError::Database(format!(
                    "journal line {} has invalid side {other}",
                    line.id
                )))
            }
        }
    }
    Ok((debit, credit))
}

fn decimal(value: Decimal) -> String {
    value.normalize().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    fn account(code: &str, normal_side: &str) -> AccountingAccountModel {
        AccountingAccountModel {
            id: Uuid::new_v4(),
            tenant_id: "hauliage".to_string(),
            legal_entity_id: Uuid::new_v4(),
            code: code.to_string(),
            name: format!("Account {code}"),
            account_type: "ASSET".to_string(),
            normal_side: normal_side.to_string(),
            control_role: None,
            currency_code: "USD".to_string(),
            is_active: true,
            created_at: NaiveDateTime::default(),
            updated_at: NaiveDateTime::default(),
        }
    }

    fn line(
        account_id: Uuid,
        side: &str,
        amount: i64,
        line_number: i32,
    ) -> AccountingJournalLineModel {
        AccountingJournalLineModel {
            id: Uuid::new_v4(),
            tenant_id: "hauliage".to_string(),
            legal_entity_id: Uuid::new_v4(),
            journal_entry_id: Uuid::new_v4(),
            line_number,
            account_id,
            side: side.to_string(),
            amount: Decimal::new(amount, 0),
            description: "test".to_string(),
            created_at: NaiveDateTime::default(),
        }
    }

    fn journal(id: Uuid, debit: i64, credit: i64) -> AccountingJournalEntryModel {
        AccountingJournalEntryModel {
            id,
            tenant_id: "hauliage".to_string(),
            legal_entity_id: Uuid::new_v4(),
            fiscal_period_id: Uuid::new_v4(),
            entry_number: "JE-2026-000001".to_string(),
            entry_date: NaiveDate::from_ymd_opt(2026, 7, 16).expect("date"),
            source_document_id: Uuid::new_v4(),
            currency_code: "USD".to_string(),
            total_debit: Decimal::new(debit, 0),
            total_credit: Decimal::new(credit, 0),
            posted_at: NaiveDateTime::default(),
            posted_by: Uuid::new_v4(),
            created_at: NaiveDateTime::default(),
        }
    }

    #[test]
    fn derives_normal_side_balances_from_one_sided_lines() {
        let debit_account = account("1100", "DEBIT");
        let credit_account = account("4000", "CREDIT");
        let lines = vec![
            line(debit_account.id, "DEBIT", 125, 1),
            line(credit_account.id, "CREDIT", 125, 2),
        ];

        let result = derive_trial_balance(&[debit_account, credit_account], &lines, false)
            .expect("balanced trial balance");

        assert_eq!(result.total_debit, Decimal::new(125, 0));
        assert_eq!(result.total_credit, Decimal::new(125, 0));
        assert_eq!(result.lines.len(), 2);
        assert!(result.lines.iter().all(|line| line["balance"] == "125"));
    }

    #[test]
    fn zero_accounts_are_optional() {
        let account = account("1100", "DEBIT");
        assert!(
            derive_trial_balance(std::slice::from_ref(&account), &[], false)
                .expect("empty balance")
                .lines
                .is_empty()
        );
        assert_eq!(
            derive_trial_balance(&[account], &[], true)
                .expect("zero-inclusive balance")
                .lines
                .len(),
            1
        );
    }

    #[test]
    fn rejects_lines_for_accounts_outside_the_currency_book() {
        let result = derive_trial_balance(&[], &[line(Uuid::new_v4(), "DEBIT", 1, 1)], false);
        assert!(matches!(result, Err(LedgerError::Database(_))));
    }

    #[test]
    fn rejects_cross_journal_cancellation() {
        let debit_journal_id = Uuid::new_v4();
        let credit_journal_id = Uuid::new_v4();
        let account_id = Uuid::new_v4();
        let mut debit_line = line(account_id, "DEBIT", 10, 1);
        debit_line.journal_entry_id = debit_journal_id;
        let mut credit_line = line(account_id, "CREDIT", 10, 1);
        credit_line.journal_entry_id = credit_journal_id;

        let result = validate_trial_balance_source(
            &[
                journal(debit_journal_id, 10, 0),
                journal(credit_journal_id, 0, 10),
            ],
            &[debit_line, credit_line],
        );

        assert!(matches!(result, Err(LedgerError::Database(_))));
    }

    #[test]
    fn validates_boundary_inputs() {
        assert!(currency("usd").is_err());
        assert!(parse_date("16/07/2026", "date").is_err());
        assert!(limit(Some(0)).is_err());
        assert_eq!(limit(None).expect("default limit"), DEFAULT_LIMIT);
    }
}
