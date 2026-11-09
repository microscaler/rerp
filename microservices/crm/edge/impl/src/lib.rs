//! RERP CRM edge: the public surface.
//!
//! - Contract: `openapi/crm/edge/openapi.yaml` (HTTP) and `asyncapi.yaml` (events); generated crate `rerp_crm_edge_gen`.
//! - Design: `openapi/crm/edge/README.md`, `openapi/crm/docs/09-api-conventions.md`, `openapi/crm/docs/10-events.md`.
//! - Work: `docs/crm/stories/` (stories CRM-EDG-*).
//!
//! The edge holds no database credentials: it validates, verifies signed tokens, rate-limits and publishes to Pub/Sub
//! (ADR 004). `services/` own that; `validators/` the request checks.

pub mod services;
pub mod validators;
