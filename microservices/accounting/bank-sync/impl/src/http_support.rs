//! Shared HTTP surface helpers for this scaffold-only service.

use brrtrouter::typed::HttpJson;
use serde_json::{json, Value};

/// Honest 501 for operations whose product behavior is not delivered.
/// Contract honesty (NFR-DOG-013): never serve generated example payloads.
#[must_use]
pub fn not_implemented(operation_id: &str) -> HttpJson<Value> {
    HttpJson::new(
        501,
        json!({
            "code": "not_implemented",
            "message": format!("operation '{operation_id}' is not implemented; this service is scaffold-only"),
            "operation_id": operation_id,
        }),
    )
}
