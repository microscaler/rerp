//! Asset Category entity
//!
//! Categories for organizing assets.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "asset_categories"]
#[skip_from_row]
#[table_comment = "Asset categories"]
#[index = "idx_asset_categories_code(code)"]
#[index = "idx_asset_categories_parent_id(parent_id)"]
pub struct AssetCategory {
    #[primary_key]
    pub id: uuid::Uuid,
    
    #[unique]
    #[indexed]
    #[column_type = "VARCHAR(50)"]
    pub code: String,
    
    #[column_type = "VARCHAR(255)"]
    pub name: String,
    
    // Hierarchical structure
    #[foreign_key = "asset_categories(id) ON DELETE SET NULL"]
    #[indexed]
    pub parent_id: Option<uuid::Uuid>,
    
    pub description: Option<String>,
    
    // Default depreciation settings for this category
    #[column_type = "VARCHAR(50)"]
    pub default_depreciation_method: Option<String>,
    
    #[default_value = "0"]
    pub default_useful_life_months: Option<i32>,
    
    #[default_value = "true"]
    pub is_active: bool,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
