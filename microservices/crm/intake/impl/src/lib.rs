//! RERP CRM intake: the quarantine (ADR 004).
//!
//! - Contract: `openapi/crm/intake/openapi.yaml` (HTTP) and `asyncapi.yaml` (events); generated crate `rerp_crm_intake_gen`.
//! - Design: `openapi/crm/intake/README.md`, `openapi/crm/docs/09-api-conventions.md`, `openapi/crm/docs/10-events.md`.
//! - Data model: `openapi/crm/docs/data-model/intake.yaml` (database `crm_intake`, schema `intake`).
//! - Work: `docs/crm/stories/` (stories CRM-INT-*).
//!
//! `controllers/` (in the binary) adapt the generated transport types to `services/`; `services/` own the use cases;
//! `models/` own the `LifeModels`; `validators/` hold rules the contract and the database cannot express.

pub mod models;
pub mod services;
pub mod validators;
