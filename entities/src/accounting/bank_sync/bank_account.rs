//! Bank Account entity
//!
//! Bank account information for synchronization and reconciliation.
//! Links to banks master table via bank_id foreign key.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "bank_accounts"]
#[skip_from_row]
#[table_comment = "Bank accounts and credit/debit cards linked to banks master table"]
#[index = "idx_bank_accounts_bank_id(bank_id)"]
#[index = "idx_bank_accounts_account_number(account_number)"]
#[index = "idx_bank_accounts_company_id(company_id)"]
#[index = "idx_bank_accounts_currency_code(currency_code)"]
#[index = "idx_bank_accounts_account_type(account_type)"]
#[index = "idx_bank_accounts_card_type(card_type)"]
#[index = "idx_bank_accounts_iban(iban)"]
#[index = "idx_bank_accounts_swift_code(swift_code)"]
pub struct BankAccount {
    #[primary_key]
    pub id: uuid::Uuid,
    
    // Bank reference (REQUIRED)
    #[foreign_key = "banks(id) ON DELETE RESTRICT"]
    #[indexed]
    pub bank_id: uuid::Uuid,
    
    // Account identification
    #[indexed]
    #[column_type = "VARCHAR(100)"]
    pub account_number: String,
    
    #[column_type = "VARCHAR(100)"]
    pub account_name: String,
    
    #[column_type = "VARCHAR(50)"]
    pub account_type: String, // CHECKING, SAVINGS, MONEY_MARKET, CERTIFICATE_OF_DEPOSIT, CREDIT_CARD, DEBIT_CARD, PREPAID_CARD, OTHER
    
    // Card information (for credit/debit cards)
    #[indexed]
    #[column_type = "VARCHAR(20)"]
    pub card_number: Option<String>, // Masked card number (last 4 digits, encrypted)
    
    #[indexed]
    #[column_type = "VARCHAR(20)"]
    pub card_type: Option<String>, // CREDIT_CARD, DEBIT_CARD, PREPAID_CARD, BANK_ACCOUNT
    
    pub card_expiry_date: Option<chrono::NaiveDate>, // Expiry date for credit cards
    
    #[column_type = "VARCHAR(255)"]
    pub cardholder_name: Option<String>, // Name on card
    
    #[column_type = "VARCHAR(50)"]
    pub card_issuer: Option<String>, // VISA, MASTERCARD, AMEX, DISCOVER, etc.
    
    #[column_type = "NUMERIC(19, 4)"]
    pub credit_limit: Option<rust_decimal::Decimal>, // Credit limit for credit cards
    
    #[column_type = "NUMERIC(19, 4)"]
    pub available_credit: Option<rust_decimal::Decimal>, // Computed: credit_limit - current_balance
    
    // Currency (per account - allows multi-currency per bank)
    #[default_value = "'USD'"]
    #[indexed]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,
    
    // International banking identifiers
    #[unique]
    #[indexed]
    #[column_type = "VARCHAR(34)"]
    pub iban: Option<String>, // International Bank Account Number (IBAN)
    
    #[indexed]
    #[column_type = "VARCHAR(11)"]
    pub swift_code: Option<String>, // SWIFT/BIC code (can override bank's default)
    
    // Account holder (if different from company)
    #[column_type = "VARCHAR(255)"]
    pub account_holder_name: Option<String>,
    
    pub partner_id: Option<uuid::Uuid>, // Optional: link to partner/customer/vendor
    
    // Balance tracking
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub current_balance: rust_decimal::Decimal,
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub reconciled_balance: rust_decimal::Decimal,
    
    pub last_reconciled_at: Option<chrono::NaiveDateTime>,
    
    // Synchronization
    #[column_type = "VARCHAR(50)"]
    pub sync_provider: Option<String>, // Bank API provider name
    
    #[column_type = "VARCHAR(255)"]
    pub sync_credentials: Option<String>, // Encrypted credentials
    
    pub last_synced_at: Option<chrono::NaiveDateTime>,
    
    // Status
    #[default_value = "true"]
    pub is_active: bool,
    
    // Multi-company
    #[indexed]
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
