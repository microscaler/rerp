//! RERP CRM sales: selling.
//!
//! - Contract: `openapi/crm/sales/openapi.yaml` (HTTP) and `asyncapi.yaml` (events); generated crate `rerp_crm_sales_gen`.
//! - Design: `openapi/crm/sales/README.md`, `openapi/crm/docs/09-api-conventions.md`, `openapi/crm/docs/10-events.md`.
//! - Data model: `openapi/crm/docs/data-model/sales.yaml` (database `crm`, schema `sales`).
//! - Work: `docs/crm/stories/` (stories CRM-SAL-*).
//!
//! `controllers/` (in the binary) adapt the generated transport types to `services/`; `services/` own the use cases;
//! `models/` own the `LifeModels`; `validators/` hold rules the contract and the database cannot express.

pub mod models;
pub mod services;
pub mod validators;
