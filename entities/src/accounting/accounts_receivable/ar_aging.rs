//! AR Aging entity
//!
//! Aging analysis snapshot for accounts receivable.

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "ar_agings"]
#[skip_from_row]
#[table_comment = "AR aging analysis snapshots"]
#[index = "idx_ar_agings_customer_id(customer_id)"]
#[index = "idx_ar_agings_aging_date(aging_date)"]
#[composite_unique = "customer_id, aging_date"]
pub struct ArAging {
    #[primary_key]
    pub id: uuid::Uuid,
    
    // Customer reference
    #[foreign_key = "customers(id) ON DELETE CASCADE"]
    #[indexed]
    pub customer_id: uuid::Uuid,
    
    // Aging snapshot date
    #[indexed]
    pub aging_date: chrono::NaiveDate,
    
    // Aging buckets
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub current: rust_decimal::Decimal, // 0-30 days
    
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
    
    // Total
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub total_outstanding: rust_decimal::Decimal,
    
    // Currency
    #[default_value = "'USD'"]
    #[column_type = "VARCHAR(3)"]
    pub currency_code: String,
    
    // Multi-company
    pub company_id: Option<uuid::Uuid>,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
}
