//! Budget Version entity
//!
//! Version control for budgets (original, revised, final).

use lifeguard_derive::LifeModel;

#[derive(LifeModel)]
#[table_name = "budget_versions"]
#[skip_from_row]
#[table_comment = "Budget versions for revision tracking"]
#[index = "idx_budget_versions_budget_id(budget_id)"]
#[index = "idx_budget_versions_version_number(version_number)"]
#[composite_unique = "budget_id, version_number"]
pub struct BudgetVersion {
    #[primary_key]
    pub id: uuid::Uuid,
    
    // Budget reference
    #[foreign_key = "budgets(id) ON DELETE CASCADE"]
    #[indexed]
    pub budget_id: uuid::Uuid,
    
    // Version identification
    #[indexed]
    #[default_value = "1"]
    pub version_number: i32,
    
    #[column_type = "VARCHAR(100)"]
    pub version_name: Option<String>, // e.g., "Original", "Revised Q1", "Final"
    
    pub description: Option<String>, // What changed in this version
    
    // Status
    #[column_type = "VARCHAR(50)"]
    pub status: String, // DRAFT, ACTIVE, SUPERSEDED
    
    pub is_current: bool, // Is this the current active version?
    
    // Version dates
    pub version_date: chrono::NaiveDate,
    
    pub superseded_at: Option<chrono::NaiveDateTime>,
    pub superseded_by_version_id: Option<uuid::Uuid>,
    
    // Totals for this version
    #[default_value = "0"]
    #[column_type = "NUMERIC(19, 4)"]
    pub total_budget_amount: rust_decimal::Decimal,
    
    // Audit
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub created_at: chrono::NaiveDateTime,
    
    #[default_expr = "CURRENT_TIMESTAMP"]
    pub updated_at: chrono::NaiveDateTime,
    
    pub created_by: Option<uuid::Uuid>,
    pub updated_by: Option<uuid::Uuid>,
}
