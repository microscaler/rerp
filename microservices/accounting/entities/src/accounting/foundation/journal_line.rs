//! Exactly one debit or credit side of a posted journal entry.

use lifeguard_derive::{LifeModel, LifeRecord};

#[derive(Debug, Clone, LifeModel, LifeRecord)]
#[table_name = "accounting_journal_lines"]
#[table_comment = "Immutable one-sided lines of posted journals"]
#[check = "one_sided_amount: (side = 'DEBIT' OR side = 'CREDIT') AND amount > 0"]
#[composite_unique = "tenant_id, legal_entity_id, journal_entry_id, line_number"]
#[index = "idx_accounting_journal_lines_scope(tenant_id, legal_entity_id)"]
#[index = "idx_accounting_journal_lines_entry(journal_entry_id)"]
#[index = "idx_accounting_journal_lines_account(account_id)"]
pub struct AccountingJournalLine {
    #[primary_key]
    pub id: uuid::Uuid,

    #[indexed]
    #[column_type = "VARCHAR(200)"]
    pub tenant_id: String,

    #[indexed]
    pub legal_entity_id: uuid::Uuid,

    #[foreign_key = "accounting_journal_entries(id) ON DELETE RESTRICT"]
    #[indexed]
    pub journal_entry_id: uuid::Uuid,

    pub line_number: i32,

    #[foreign_key = "accounting_accounts(id) ON DELETE RESTRICT"]
    #[indexed]
    pub account_id: uuid::Uuid,

    #[column_type = "VARCHAR(6)"]
    pub side: String,

    #[column_type = "NUMERIC(19, 6)"]
    pub amount: rust_decimal::Decimal,

    #[column_type = "VARCHAR(1000)"]
    pub description: String,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
}
