//! Immutable balanced journal entry produced with a posted document.

use lifeguard_derive::{LifeModel, LifeRecord};

#[derive(Debug, Clone, LifeModel, LifeRecord)]
#[table_name = "accounting_journal_entries"]
#[table_comment = "Immutable posted journal entry headers"]
#[check = "balanced_journal: total_debit = total_credit"]
#[composite_unique = "tenant_id, legal_entity_id, entry_number"]
#[composite_unique = "tenant_id, legal_entity_id, source_document_id"]
#[composite_unique = "tenant_id, legal_entity_id, id"]
#[index = "idx_accounting_journal_scope(tenant_id, legal_entity_id)"]
#[index = "idx_accounting_journal_period(fiscal_period_id)"]
#[index = "idx_accounting_journal_date(entry_date)"]
pub struct AccountingJournalEntry {
    #[primary_key]
    pub id: uuid::Uuid,

    #[indexed]
    #[column_type = "VARCHAR(200)"]
    pub tenant_id: String,

    #[indexed]
    pub legal_entity_id: uuid::Uuid,

    #[foreign_key = "accounting_fiscal_periods(id) ON DELETE RESTRICT"]
    #[indexed]
    pub fiscal_period_id: uuid::Uuid,

    #[column_type = "VARCHAR(100)"]
    pub entry_number: String,

    #[indexed]
    pub entry_date: chrono::NaiveDate,

    #[foreign_key = "accounting_posted_documents(id) ON DELETE RESTRICT"]
    pub source_document_id: uuid::Uuid,

    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,

    #[column_type = "NUMERIC(19, 6)"]
    pub total_debit: rust_decimal::Decimal,

    #[column_type = "NUMERIC(19, 6)"]
    pub total_credit: rust_decimal::Decimal,

    pub posted_at: chrono::NaiveDateTime,

    pub posted_by: uuid::Uuid,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
}
