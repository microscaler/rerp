//! Asset entity
//!
//! Fixed assets with depreciation tracking.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "assets"]
#[skip_from_row]
#[table_comment = "Fixed assets"]
#[index = "idx_assets_asset_number(asset_number)"]
#[index = "idx_assets_category_id(category_id)"]
#[index = "idx_assets_status(status)"]
#[index = "idx_assets_company_id(company_id)"]
pub struct Asset {
    #[primary_key]
    pub id: uuid::Uuid,

    // Asset identification
    #[unique]
    #[indexed]
    #[column_type = "VARCHAR(100)"]
    pub asset_number: String,

    #[column_type = "VARCHAR(255)"]
    pub name: String,

    pub description: Option<String>,

    // Category
    #[foreign_key = "asset_categories(id) ON DELETE RESTRICT"]
    #[indexed]
    pub category_id: uuid::Uuid,

    // Financial information
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub purchase_cost: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub current_value: rust_decimal::Decimal, // Book value after depreciation

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub accumulated_depreciation: rust_decimal::Decimal,

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub salvage_value: rust_decimal::Decimal, // Residual value

    // Dates
    pub purchase_date: chrono::NaiveDate,
    pub in_service_date: Option<chrono::NaiveDate>, // When depreciation starts
    pub disposal_date: Option<chrono::NaiveDate>,

    // Depreciation
    #[column_type = "VARCHAR(50)"]
    pub depreciation_method: Option<String>, // STRAIGHT_LINE, DECLINING_BALANCE, UNITS_OF_PRODUCTION

    #[default_value = "0"]
    pub useful_life_months: i32, // Expected useful life in months

    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 6)"]
    pub depreciation_rate: rust_decimal::Decimal, // Annual depreciation rate

    // Status
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub status: String, // ACTIVE, DEPRECIATING, FULLY_DEPRECIATED, DISPOSED, IMPAIRED

    // Location
    #[column_type = "VARCHAR(255)"]
    pub location: Option<String>,

    // GL Account references
    pub asset_account_id: Option<uuid::Uuid>, // GL account for asset
    pub depreciation_expense_account_id: Option<uuid::Uuid>, // GL account for depreciation expense
    pub accumulated_depreciation_account_id: Option<uuid::Uuid>, // GL account for accumulated depreciation

    // Currency
    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,

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
