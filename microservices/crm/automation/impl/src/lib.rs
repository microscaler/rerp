//! RERP CRM automation: what happens next.
//!
//! - Contract: `openapi/crm/automation/openapi.yaml` (HTTP) and `asyncapi.yaml` (events); generated crate `rerp_crm_automation_gen`.
//! - Design: `openapi/crm/automation/README.md`, `openapi/crm/docs/09-api-conventions.md`, `openapi/crm/docs/10-events.md`.
//! - Data model: `openapi/crm/docs/data-model/automation.yaml` (database `crm`, schema `automation`).
//! - Work: `docs/crm/stories/` (stories CRM-AUT-*).
//!
//! `controllers/` (in the binary) adapt the generated transport types to `services/`; `services/` own the use cases;
//! `models/` own the LifeModels; `validators/` hold rules the contract and the database cannot express.

pub mod models;
pub mod services;
pub mod validators;
