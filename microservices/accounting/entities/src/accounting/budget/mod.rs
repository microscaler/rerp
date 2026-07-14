//! Budget Management Service Entities
//!
//! This module contains entities for budget planning and tracking including:
//! - Budget periods
//! - Budget line items
//! - Budget versions
//! - Budget actuals

// Preserve the public entity path used by the registry and generated migrations.
#[allow(clippy::module_inception)]
pub mod budget;
pub mod budget_actual;
pub mod budget_line_item;
pub mod budget_period;
pub mod budget_version;

pub use budget::Budget;
pub use budget_actual::BudgetActual;
pub use budget_line_item::BudgetLineItem;
pub use budget_period::BudgetPeriod;
pub use budget_version::BudgetVersion;
