//! Shared HTTP adaptation for the accounts-receivable runtime.

use brrtrouter::typed::HttpJson;
use lifeguard::{ExclusivePrimaryLifeExecutor, LifeError, SessionContext};
use serde_json::{json, Value};

use crate::ledger::ArError;

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

pub fn identity_context(claims: Option<&Value>) -> Result<SessionContext, HttpJson<Value>> {
    crate::identity::from_validated_claims(claims).map_err(|error| {
        let (status, code, message) = match error {
            crate::identity::IdentityError::MissingValidatedClaims => {
                (401, "unauthorized", "Authentication is required")
            }
            _ => (
                403,
                "invalid_identity_context",
                "The authenticated identity context is incomplete or inconsistent",
            ),
        };
        HttpJson::new(status, json!({ "code": code, "message": message }))
    })
}

pub fn with_ar_transaction<T>(
    context: &SessionContext,
    operation: impl FnOnce(&ExclusivePrimaryLifeExecutor<'_>) -> Result<T, ArError>,
) -> Result<T, ArError> {
    let database = crate::database::db().map_err(ArError::Database)?;
    let mut domain_error = None;
    let mut output = None;
    let transaction =
        database
            .pool()
            .with_session_transaction(context, |executor| match operation(executor) {
                Ok(value) => {
                    output = Some(value);
                    Ok(())
                }
                Err(error) => {
                    domain_error = Some(error);
                    Err(LifeError::Other(
                        "accounts-receivable read failed; transaction rolled back".to_string(),
                    ))
                }
            });
    match (transaction, domain_error, output) {
        (Ok(()), None, Some(value)) => Ok(value),
        (Ok(()), None, None) => Err(ArError::Database(
            "transaction completed without an accounts-receivable result".to_string(),
        )),
        (Err(error), None, _) => Err(ArError::Database(error.to_string())),
        (_, Some(error), _) => Err(error),
    }
}

pub fn ar_error(error: ArError) -> HttpJson<Value> {
    // Error codes come from the contract's ErrorResponse enum (the
    // response validator rejects anything else with a 500 wrap).
    let (status, code, message) = match &error {
        ArError::Validation(_) => (400, "UNKNOWN", error.to_string()),
        ArError::Policy(_) => (403, "FORBIDDEN", error.to_string()),
        ArError::NotFound => (404, "NOT_FOUND", "Open item not found".to_string()),
        ArError::Database(_) => {
            eprintln!("accounts-receivable runtime database failure: {error}");
            (
                500,
                "SERVER_ERROR",
                "The accounting data source is temporarily unavailable".to_string(),
            )
        }
    };
    HttpJson::new(status, json!({ "code": code, "message": message }))
}
