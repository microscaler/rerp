//! Asset Depreciation entity
//!
//! Depreciation schedule entries for assets.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "asset_depreciations"]
#[skip_from_row]
#[table_comment = "Asset depreciation schedule entries"]
#[index = "idx_asset_depreciations_asset_id(asset_id)"]
#[index = "idx_asset_depreciations_period_start(period_start)"]
#[composite_unique = "asset_id, period_start"]
pub struct AssetDepreciation {
    #[primary_key]
    pub id: uuid::Uuid,
    
    // Asset reference
    #[foreign_key = "assets(id) ON DELETE CASCADE"]
    #[indexed]
    pub asset_id: uuid::Uuid,
    
    // Depreciation period
    #[indexed]
    pub period_start: chrono::NaiveDate,
    
    pub period_end: chrono::NaiveDate,
    
    // Depreciation amount
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub depreciation_amount: rust_decimal::Decimal,
    
    // Accumulated totals
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub accumulated_depreciation: rust_decimal::Decimal, // Total accumulated to date
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub book_value: rust_decimal::Decimal, // purchase_cost - accumulated_depreciation
    
    // Journal entry reference (if posted)
    pub journal_entry_id: Option<uuid::Uuid>, // GL journal entry for this depreciation
    
    // Status
    #[column_type = "VARCHAR(50)"]
    pub status: String, // SCHEDULED, POSTED, SKIPPED
    
    pub posted_at: Option<chrono::NaiveDateTime>,
    pub posted_by: Option<uuid::Uuid>,
    
    // Currency
    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
