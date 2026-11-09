//! RERP CRM messaging: what we send.
//!
//! - Contract: `openapi/crm/messaging/openapi.yaml` (HTTP) and `asyncapi.yaml` (events); generated crate `rerp_crm_messaging_gen`.
//! - Design: `openapi/crm/messaging/README.md`, `openapi/crm/docs/09-api-conventions.md`, `openapi/crm/docs/10-events.md`.
//! - Data model: `openapi/crm/docs/data-model/messaging.yaml` (database `crm`, schema `messaging`).
//! - Work: `docs/crm/stories/` (stories CRM-MES-*).
//!
//! `controllers/` (in the binary) adapt the generated transport types to `services/`; `services/` own the use cases;
//! `models/` own the `LifeModels`; `validators/` hold rules the contract and the database cannot express.

pub mod models;
pub mod services;
pub mod validators;
