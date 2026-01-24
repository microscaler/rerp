//! Bank entity
//!
//! Master list of banks with complete information.

use lifeguard_derive::LifeModel;
use serde_json::Value;

#[derive(LifeModel)]
#[table_name = "banks"]
#[skip_from_row]
#[table_comment = "Master list of banks"]
#[index = "idx_banks_name(name)"]
#[index = "idx_banks_country_code(country_code)"]
#[index = "idx_banks_is_active(is_active)"]
pub struct Bank {
    #[primary_key]
    pub id: uuid::Uuid,
    
    // Bank identification
    #[indexed]
    #[column_type = "VARCHAR(255)"]
    pub name: String,
    
    #[unique]
    #[indexed]
    #[column_type = "VARCHAR(11)"]
    pub bic: Option<String>, // Bank Identifier Code (BIC) - same as SWIFT
    
    #[unique]
    #[indexed]
    #[column_type = "VARCHAR(11)"]
    pub swift_code: Option<String>, // SWIFT code (alias for BIC, for clarity)
    
    #[column_type = "VARCHAR(50)"]
    pub routing_number: Option<String>, // US routing number, etc.
    
    // Address information
    #[column_type = "VARCHAR(255)"]
    pub street: Option<String>,
    
    #[column_type = "VARCHAR(255)"]
    pub street2: Option<String>,
    
    #[column_type = "VARCHAR(100)"]
    pub city: Option<String>,
    
    #[column_type = "VARCHAR(100)"]
    pub state: Option<String>,
    
    #[column_type = "VARCHAR(20)"]
    pub zip: Option<String>,
    
    #[indexed]
    #[column_type = "VARCHAR(3)"]
    pub country_code: Option<String>, // ISO country code
    
    // Contact information
    #[column_type = "VARCHAR(255)"]
    pub email: Option<String>,
    
    #[column_type = "VARCHAR(50)"]
    pub phone: Option<String>,
    
    #[column_type = "VARCHAR(255)"]
    pub website: Option<String>,
    
    // Status
    #[default_value = "true"]
    #[indexed]
    pub is_active: bool,
    
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
