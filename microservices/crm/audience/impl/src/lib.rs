//! RERP CRM audience: who we can reach.
//!
//! - Contract: `openapi/crm/audience/openapi.yaml` (HTTP) and `asyncapi.yaml` (events); generated crate `rerp_crm_audience_gen`.
//! - Design: `openapi/crm/audience/README.md`, `openapi/crm/docs/09-api-conventions.md`, `openapi/crm/docs/10-events.md`.
//! - Data model: `openapi/crm/docs/data-model/audience.yaml` (database `crm`, schema `audience`).
//! - Work: `docs/crm/stories/` (stories CRM-AUD-*).
//!
//! `controllers/` (in the binary) adapt the generated transport types to `services/`; `services/` own the use cases;
//! `models/` own the `LifeModels`; `validators/` hold rules the contract and the database cannot express.

pub mod models;
pub mod services;
pub mod validators;
