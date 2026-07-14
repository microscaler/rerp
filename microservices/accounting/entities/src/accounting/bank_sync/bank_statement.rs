//! Bank Statement entity
//!
//! Bank statements imported from banks.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "bank_statements"]
#[skip_from_row]
#[table_comment = "Bank statements imported from banks"]
#[index = "idx_bank_statements_bank_account_id(bank_account_id)"]
#[index = "idx_bank_statements_statement_date(statement_date)"]
#[index = "idx_bank_statements_status(status)"]
pub struct BankStatement {
    #[primary_key]
    pub id: uuid::Uuid,

    // Bank account reference
    #[foreign_key = "bank_accounts(id) ON DELETE CASCADE"]
    #[indexed]
    pub bank_account_id: uuid::Uuid,

    // Statement identification
    #[unique]
    #[column_type = "VARCHAR(100)"]
    pub statement_number: Option<String>, // Statement number from bank

    #[indexed]
    pub statement_date: chrono::NaiveDate, // Statement date/period end

    pub period_start: Option<chrono::NaiveDate>,
    pub period_end: Option<chrono::NaiveDate>,

    // Balances
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub opening_balance: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub closing_balance: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub total_debits: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub total_credits: rust_decimal::Decimal,

    // Transaction count
    #[default_value = "0"]
    pub transaction_count: i32,

    // Status
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub status: String, // IMPORTED, PROCESSING, RECONCILED, ERROR

    pub reconciled_at: Option<chrono::NaiveDateTime>,
    pub reconciled_by: Option<uuid::Uuid>,

    // Import information
    #[column_type = "VARCHAR(50)"]
    pub import_format: Option<String>, // OFX, CSV, MT940, etc.

    #[column_type = "VARCHAR(255)"]
    pub import_source: Option<String>, // File name, API endpoint, etc.

    pub imported_at: Option<chrono::NaiveDateTime>,
    pub imported_by: Option<uuid::Uuid>,

    // Currency
    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,

    // Multi-company
    pub company_id: Option<uuid::Uuid>,

    // Metadata
    #[column_type = "JSONB"]
    pub metadata: Option<Value>, // Raw statement data

    // Audit
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,

    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,

    pub created_by: Option<uuid::Uuid>,
    pub updated_by: Option<uuid::Uuid>,
}
