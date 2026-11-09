//! RERP CRM workspace: how the workspace is run.
//!
//! - Contract: `openapi/crm/workspace/openapi.yaml` (HTTP) and `asyncapi.yaml` (events); generated crate `rerp_crm_workspace_gen`.
//! - Design: `openapi/crm/workspace/README.md`, `openapi/crm/docs/09-api-conventions.md`, `openapi/crm/docs/10-events.md`.
//! - Data model: `openapi/crm/docs/data-model/workspace.yaml` (database `crm`, schema `workspace`).
//! - Work: `docs/crm/stories/` (stories CRM-WOR-*).
//!
//! `controllers/` (in the binary) adapt the generated transport types to `services/`; `services/` own the use cases;
//! `models/` own the `LifeModels`; `validators/` hold rules the contract and the database cannot express.

pub mod models;
pub mod services;
pub mod validators;
