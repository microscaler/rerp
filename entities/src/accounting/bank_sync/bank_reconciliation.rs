//! Bank Reconciliation entity
//!
//! Reconciliation records linking bank statements to accounting records.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "bank_reconciliations"]
#[skip_from_row]
#[table_comment = "Bank reconciliation records"]
#[index = "idx_bank_reconciliations_bank_account_id(bank_account_id)"]
#[index = "idx_bank_reconciliations_statement_id(statement_id)"]
#[index = "idx_bank_reconciliations_status(status)"]
pub struct BankReconciliation {
    #[primary_key]
    pub id: uuid::Uuid,
    
    // Bank account reference
    #[foreign_key = "bank_accounts(id) ON DELETE CASCADE"]
    #[indexed]
    pub bank_account_id: uuid::Uuid,
    
    // Statement reference
    #[foreign_key = "bank_statements(id) ON DELETE CASCADE"]
    #[indexed]
    pub statement_id: uuid::Uuid,
    
    // Reconciliation period
    pub reconciliation_date: chrono::NaiveDate,
    
    // Balances
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub book_balance: rust_decimal::Decimal, // Balance from accounting system
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub bank_balance: rust_decimal::Decimal, // Balance from bank statement
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub difference: rust_decimal::Decimal, // book_balance - bank_balance
    
    // Outstanding items
    #[default_value = "0"]
    pub outstanding_deposits_count: i32,
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub outstanding_deposits_amount: rust_decimal::Decimal,
    
    #[default_value = "0"]
    pub outstanding_withdrawals_count: i32,
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub outstanding_withdrawals_amount: rust_decimal::Decimal,
    
    // Status
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub status: String, // DRAFT, IN_PROGRESS, RECONCILED, DISPUTED
    
    pub reconciled_at: Option<chrono::NaiveDateTime>,
    pub reconciled_by: Option<uuid::Uuid>,
    
    // Notes
    pub notes: Option<String>,
    
    // Currency
    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,
    
    // Multi-company
    pub company_id: Option<uuid::Uuid>,
    
    // Metadata
    #[column_type = "JSONB"]
    pub metadata: Option<Value>,
    
    // Audit
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
    
    pub created_by: Option<uuid::Uuid>,
    pub updated_by: Option<uuid::Uuid>,
}
