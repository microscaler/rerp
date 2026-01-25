//! Asset Transaction entity
//!
//! Transactions affecting assets (purchase, sale, disposal, transfer, impairment).

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "asset_transactions"]
#[skip_from_row]
#[table_comment = "Asset transactions (purchase, sale, disposal, etc.)"]
#[index = "idx_asset_transactions_asset_id(asset_id)"]
#[index = "idx_asset_transactions_transaction_date(transaction_date)"]
#[index = "idx_asset_transactions_transaction_type(transaction_type)"]
pub struct AssetTransaction {
    #[primary_key]
    pub id: uuid::Uuid,

    // Asset reference
    #[foreign_key = "assets(id) ON DELETE CASCADE"]
    #[indexed]
    pub asset_id: uuid::Uuid,

    // Transaction details
    #[indexed]
    pub transaction_date: chrono::NaiveDate,

    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub transaction_type: String, // PURCHASE, SALE, DISPOSAL, TRANSFER, IMPAIRMENT, ADJUSTMENT

    // Amounts
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub transaction_amount: rust_decimal::Decimal,

    // For sale/disposal
    pub sale_proceeds: Option<rust_decimal::Decimal>,
    pub gain_loss: Option<rust_decimal::Decimal>, // Gain or loss on disposal

    // For transfer
    pub transfer_to_location: Option<String>,
    pub transfer_to_company_id: Option<uuid::Uuid>,

    // For impairment
    pub impairment_amount: Option<rust_decimal::Decimal>,
    pub impairment_reason: Option<String>,

    // Journal entry reference
    pub journal_entry_id: Option<uuid::Uuid>, // GL journal entry for this transaction

    // Reference numbers
    #[column_type = "VARCHAR(100)"]
    pub reference_number: Option<String>, // PO number, invoice number, etc.

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
