//! Shared HTTP adaptation without introducing another runtime abstraction.

use brrtrouter::typed::HttpJson;
use lifeguard::{ExclusivePrimaryLifeExecutor, LifeError, SessionContext};
use serde_json::{json, Value};

use crate::posting::PostingError;

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

pub fn with_accounting_transaction<T>(
    context: &SessionContext,
    operation: impl FnOnce(&ExclusivePrimaryLifeExecutor<'_>) -> Result<T, PostingError>,
) -> Result<T, PostingError> {
    let database = crate::database::db().map_err(PostingError::Database)?;
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
                        "accounting operation rejected; transaction rolled back".to_string(),
                    ))
                }
            });
    match (transaction, domain_error, output) {
        (Ok(()), None, Some(value)) => Ok(value),
        (Ok(()), None, None) => Err(PostingError::Database(
            "transaction completed without an accounting result".to_string(),
        )),
        (Err(error), None, _) => Err(PostingError::Database(error.to_string())),
        (_, Some(error), _) => Err(error),
    }
}

pub fn posting_error(error: PostingError) -> HttpJson<Value> {
    let (status, code, message) = match &error {
        PostingError::Validation(_) => (400, "validation_error", error.to_string()),
        PostingError::Policy(_) => (422, "accounting_policy_error", error.to_string()),
        PostingError::Conflict => (
            409,
            "idempotency_conflict",
            "The idempotency key was already used for different commercial facts".to_string(),
        ),
        PostingError::NotFound => (
            404,
            "not_found",
            "The accounting resource was not found".to_string(),
        ),
        PostingError::Database(_) => {
            eprintln!("invoice runtime database failure: {error}");
            (
                500,
                "accounting_persistence_error",
                "The accounting operation could not be completed".to_string(),
            )
        }
    };
    HttpJson::new(status, json!({ "code": code, "message": message }))
}
