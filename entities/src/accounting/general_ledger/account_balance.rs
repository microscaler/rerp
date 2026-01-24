//! Account Balance entity
//!
//! Current balance for each account (denormalized for performance).

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "account_balances"]
#[skip_from_row]  // Skip FromRow generation - types don't implement FromSql yet
#[table_comment = "Denormalized account balances for performance"]
#[composite_unique = "account_id, fiscal_period_id, balance_date, currency_code, company_id"]
#[index = "idx_account_balances_account_id(account_id)"]
#[index = "idx_account_balances_fiscal_period_id(fiscal_period_id)"]
#[index = "idx_account_balances_balance_date(balance_date)"]
#[index = "idx_account_balances_company_id(company_id)"]
pub struct AccountBalance {
    #[primary_key]
    pub id: uuid::Uuid,
    
    // Foreign key to accounts
    #[foreign_key = "accounts(id) ON DELETE CASCADE"]
    #[indexed]
    pub account_id: uuid::Uuid,
    
    pub fiscal_period_id: uuid::Uuid,
    
    #[indexed]
    pub balance_date: chrono::NaiveDate,
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub debit_balance: rust_decimal::Decimal,
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub credit_balance: rust_decimal::Decimal,
    
    // Generated column: net_balance NUMERIC(19, 4) NOT NULL GENERATED ALWAYS AS (debit_balance - credit_balance) STORED
    // Note: This is a PostgreSQL generated column. We'll handle it in SQL generation
    // by checking for a special pattern or attribute. For now, we'll add it manually in SQL.
    
    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,
    
    #[indexed]
    pub company_id: Option<uuid::Uuid>,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
