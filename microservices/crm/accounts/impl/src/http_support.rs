//! Staff reads under the caller's own Sesame RLS session.

use brrtrouter::typed::HttpJson;
use lifeguard::{ExclusivePrimaryLifeExecutor, LifeError};
use serde_json::{json, Value};

pub fn fail(status: u16, message: &str) -> HttpJson<Value> {
    HttpJson::new(status, json!({ "code": status, "message": message }))
}

/// Run `operation` in a transaction scoped to the caller's tenant (RLS).
pub fn with_caller<T>(
    claims: Option<&Value>,
    operation: impl FnOnce(&ExclusivePrimaryLifeExecutor<'_>) -> Result<T, LifeError>,
) -> Result<T, HttpJson<Value>> {
    let context = crate::identity::from_validated_claims(claims)
        .map_err(|_| fail(403, "the authenticated identity context is incomplete"))?;
    let db = crate::database::db().map_err(|e| {
        eprintln!("accounts: database unavailable: {e}");
        fail(503, "the customer database is unavailable")
    })?;
    db.pool()
        .with_session_transaction(&context, operation)
        .map_err(|e| {
            eprintln!("accounts: query failed: {e}");
            fail(500, "the customer query failed")
        })
}
