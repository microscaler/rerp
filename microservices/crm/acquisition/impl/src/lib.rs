//! RERP CRM acquisition: where contacts come from and what they are worth.
//!
//! - Contract: `openapi/crm/acquisition/openapi.yaml` (HTTP) and `asyncapi.yaml` (events); generated crate `rerp_crm_acquisition_gen`.
//! - Design: `openapi/crm/acquisition/README.md`, `openapi/crm/docs/09-api-conventions.md`, `openapi/crm/docs/10-events.md`.
//! - Data model: `openapi/crm/docs/data-model/acquisition.yaml` (database `crm`, schema `acquisition`).
//! - Work: `docs/crm/stories/` (stories CRM-ACQ-*).
//!
//! `controllers/` (in the binary) adapt the generated transport types to `services/`; `services/` own the use cases;
//! `models/` own the LifeModels; `validators/` hold rules the contract and the database cannot express.

pub mod models;
pub mod services;
pub mod validators;
