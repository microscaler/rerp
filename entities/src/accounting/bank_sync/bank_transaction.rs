//! Bank Transaction entity
//!
//! Individual transactions imported from bank statements.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "bank_transactions"]
#[skip_from_row]
#[table_comment = "Bank transactions imported from statements"]
#[index = "idx_bank_transactions_bank_account_id(bank_account_id)"]
#[index = "idx_bank_transactions_transaction_date(transaction_date)"]
#[index = "idx_bank_transactions_reference(reference)"]
#[index = "idx_bank_transactions_status(status)"]
pub struct BankTransaction {
    #[primary_key]
    pub id: uuid::Uuid,
    
    // Bank account reference
    #[foreign_key = "bank_accounts(id) ON DELETE CASCADE"]
    #[indexed]
    pub bank_account_id: uuid::Uuid,
    
    // Transaction details
    #[indexed]
    pub transaction_date: chrono::NaiveDate,
    
    #[indexed]
    #[column_type = "VARCHAR(100)"]
    pub reference: Option<String>, // Transaction reference from bank
    
    #[column_type = "VARCHAR(255)"]
    pub description: String,
    
    // Amount
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub amount: rust_decimal::Decimal, // Positive for deposits, negative for withdrawals
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub balance_after: rust_decimal::Decimal, // Account balance after this transaction
    
    // Transaction type
    #[column_type = "VARCHAR(50)"]
    pub transaction_type: Option<String>, // DEBIT, CREDIT, FEE, INTEREST, etc.
    
    // Matching and reconciliation
    #[column_type = "VARCHAR(50)"]
    pub status: String, // UNMATCHED, MATCHED, RECONCILED, IGNORED
    
    pub matched_payment_id: Option<uuid::Uuid>, // Matched AR/AP payment
    pub matched_at: Option<chrono::NaiveDateTime>,
    
    pub reconciled_statement_id: Option<uuid::Uuid>, // Statement this was reconciled in
    
    // Bank statement reference
    pub statement_id: Option<uuid::Uuid>, // Statement this transaction came from
    pub statement_line_number: Option<i32>,
    
    // Currency
    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,
    
    // Metadata
    #[column_type = "JSONB"]
    pub metadata: Option<Value>, // Raw bank data, additional fields
    
    // Audit
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
