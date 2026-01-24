//! Journal Entry entity
//!
//! Double-entry bookkeeping records.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "journal_entries"]
#[skip_from_row]  // Skip FromRow generation - types don't implement FromSql yet
#[table_comment = "Double-entry journal entries"]
#[check = "balanced_entry: total_debit = total_credit"]
#[index = "idx_journal_entries_entry_number(entry_number)"]
#[index = "idx_journal_entries_entry_date(entry_date)"]
#[index = "idx_journal_entries_status(status)"]
#[index = "idx_journal_entries_source(source_type, source_id)"]
#[index = "idx_journal_entries_fiscal_period_id(fiscal_period_id)"]
#[index = "idx_journal_entries_company_id(company_id)"]
pub struct JournalEntry {
    #[primary_key]
    pub id: uuid::Uuid,
    
    #[unique]
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub entry_number: String,
    
    #[indexed]
    pub entry_date: chrono::NaiveDate,
    
    pub description: String,
    
    #[column_type = "VARCHAR(100)"]
    pub reference_number: Option<String>, // External reference (invoice number, etc.)
    
    #[column_type = "VARCHAR(50)"]
    pub source_type: Option<String>, // MANUAL, INVOICE, PAYMENT, ADJUSTMENT, etc.
    
    pub source_id: Option<uuid::Uuid>, // Reference to source document
    
    pub fiscal_period_id: Option<uuid::Uuid>, // Reference to fiscal period
    
    #[default_value = "'DRAFT'"]
    #[indexed]
    #[column_type = "VARCHAR(20)"]
    pub status: String, // DRAFT, POSTED, REVERSED
    
    pub posted_at: Option<chrono::NaiveDateTime>,
    
    pub posted_by: Option<uuid::Uuid>, // User who posted the entry
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub total_debit: rust_decimal::Decimal,
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub total_credit: rust_decimal::Decimal,
    
    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,
    
    #[indexed]
    pub company_id: Option<uuid::Uuid>, // Multi-company support
    
    pub metadata: Option<Value>, // JSONB
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
    
    pub created_by: Option<uuid::Uuid>,
    
    pub updated_by: Option<uuid::Uuid>,
}
