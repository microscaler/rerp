//! AP Aging entity
//!
//! Aging analysis snapshot for accounts payable.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "ap_agings"]
#[skip_from_row]
#[table_comment = "AP aging analysis snapshots"]
#[index = "idx_ap_agings_vendor_id(vendor_id)"]
#[index = "idx_ap_agings_aging_date(aging_date)"]
#[composite_unique = "vendor_id, aging_date"]
pub struct ApAging {
    #[primary_key]
    pub id: uuid::Uuid,
    
    #[foreign_key = "vendors(id) ON DELETE CASCADE"]
    #[indexed]
    pub vendor_id: uuid::Uuid,
    
    #[indexed]
    pub aging_date: chrono::NaiveDate,
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub current: rust_decimal::Decimal,
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub days_31_60: rust_decimal::Decimal,
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub days_61_90: rust_decimal::Decimal,
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub days_91_120: rust_decimal::Decimal,
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub over_120: rust_decimal::Decimal,
    
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub total_outstanding: rust_decimal::Decimal,
    
    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,
    
    pub company_id: Option<uuid::Uuid>,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
