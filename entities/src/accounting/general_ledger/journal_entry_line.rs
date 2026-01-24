//! Journal Entry Line entity
//!
//! Individual debit/credit lines in a journal entry.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "journal_entry_lines"]
#[skip_from_row]  // Skip FromRow generation - types don't implement FromSql yet
#[table_comment = "Individual debit/credit lines in journal entries"]
#[check = "debit_or_credit: (debit_amount > 0 AND credit_amount = 0) OR (debit_amount = 0 AND credit_amount > 0)"]
#[index = "idx_journal_entry_lines_journal_entry_id(journal_entry_id)"]
#[index = "idx_journal_entry_lines_account_id(account_id)"]
#[index = "idx_journal_entry_lines_line_number(journal_entry_id, line_number)"]
pub struct JournalEntryLine {
    #[primary_key]
    pub id: uuid::Uuid,
    
    // Foreign key to journal_entries
    #[foreign_key = "journal_entries(id) ON DELETE CASCADE"]
    #[indexed]
    pub journal_entry_id: uuid::Uuid,
    
    // Foreign key to accounts
    #[foreign_key = "accounts(id) ON DELETE RESTRICT"]
    #[indexed]
    pub account_id: uuid::Uuid,
    
    pub line_number: i32,
    
    pub description: Option<String>,
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub debit_amount: rust_decimal::Decimal,
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub credit_amount: rust_decimal::Decimal,
    
    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,
    
    #[default_value = "1.0"]
    #[column_type = "NUMERIC(19, 6)"]
    pub exchange_rate: Option<rust_decimal::Decimal>, // For multi-currency
    
    #[column_type = "NUMERIC(19, 4)"]
    pub base_debit_amount: Option<rust_decimal::Decimal>, // Base currency amount
    
    #[column_type = "NUMERIC(19, 4)"]
    pub base_credit_amount: Option<rust_decimal::Decimal>, // Base currency amount
    
    #[column_type = "JSONB"]
    pub metadata: Option<Value>, // JSONB
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
}
