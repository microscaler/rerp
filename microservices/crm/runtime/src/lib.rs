//! What every RERP CRM service shares. Each module is specified by its story in docs/crm/stories/; until a story
//! is done its module is empty, and no service may grow its own copy of what belongs here.

pub mod audit;
pub mod conditions;
pub mod context;
pub mod errors;
pub mod events;
pub mod idempotency;
pub mod tokens;
