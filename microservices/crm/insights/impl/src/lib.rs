//! RERP CRM insights: what it adds up to.
//!
//! - Contract: `openapi/crm/insights/openapi.yaml` (HTTP) and `asyncapi.yaml` (events); generated crate `rerp_crm_insights_gen`.
//! - Design: `openapi/crm/insights/README.md`, `openapi/crm/docs/09-api-conventions.md`, `openapi/crm/docs/10-events.md`.
//! - Data model: `openapi/crm/docs/data-model/insights.yaml` (database `crm_insights`, schema `insights`).
//! - Work: `docs/crm/stories/` (stories CRM-INS-*).
//!
//! `controllers/` (in the binary) adapt the generated transport types to `services/`; `services/` own the use cases;
//! `models/` own the LifeModels; `validators/` hold rules the contract and the database cannot express.

pub mod models;
pub mod services;
pub mod validators;
