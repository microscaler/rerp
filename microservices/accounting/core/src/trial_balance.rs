use crate::{AccountingError, JournalEntry, JournalSide};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrialBalanceAccount {
    pub account_id: Uuid,
    pub debit: Decimal,
    pub credit: Decimal,
    pub closing_debit: Decimal,
    pub closing_credit: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrialBalance {
    pub tenant_id: String,
    pub legal_entity_id: Uuid,
    pub accounts: Vec<TrialBalanceAccount>,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
}

/// Derive an auditable trial balance from posted journal lines.
pub fn derive_trial_balance(
    tenant_id: &str,
    legal_entity_id: Uuid,
    journals: &[JournalEntry],
) -> Result<TrialBalance, AccountingError> {
    let mut accounts: BTreeMap<Uuid, (Decimal, Decimal)> = BTreeMap::new();
    let mut total_debit = Decimal::ZERO;
    let mut total_credit = Decimal::ZERO;

    for journal in journals {
        if journal.tenant_id != tenant_id {
            return Err(AccountingError::TenantMismatch);
        }
        if journal.legal_entity_id != legal_entity_id {
            return Err(AccountingError::LegalEntityMismatch);
        }
        journal.validate()?;
        for line in &journal.lines {
            let balance = accounts
                .entry(line.account_id)
                .or_insert((Decimal::ZERO, Decimal::ZERO));
            match line.side {
                JournalSide::Debit => {
                    balance.0 += line.amount;
                    total_debit += line.amount;
                }
                JournalSide::Credit => {
                    balance.1 += line.amount;
                    total_credit += line.amount;
                }
            }
        }
    }

    if total_debit != total_credit {
        return Err(AccountingError::UnbalancedJournal);
    }

    let accounts = accounts
        .into_iter()
        .map(|(account_id, (debit, credit))| {
            let net = debit - credit;
            TrialBalanceAccount {
                account_id,
                debit,
                credit,
                closing_debit: net.max(Decimal::ZERO),
                closing_credit: (-net).max(Decimal::ZERO),
            }
        })
        .collect();

    Ok(TrialBalance {
        tenant_id: tenant_id.to_owned(),
        legal_entity_id,
        accounts,
        total_debit,
        total_credit,
    })
}
