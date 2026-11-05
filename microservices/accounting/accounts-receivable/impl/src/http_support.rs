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
                (401, "UNAUTHORIZED", "Authentication is required")
            }
            _ => (
                403,
                "FORBIDDEN",
                "The authenticated identity context is incomplete or inconsistent",
            ),
        };
        HttpJson::new(status, json!({ "code": code, "message": message }))
    })
}

/// Identity gate for accounts-receivable mutations (Epic F2 write paths):
/// authentication alone is not enough — the exact write permission must be
/// granted, mirroring the GL `identity_write_context` discipline. RLS keeps
/// the row scope honest; this keeps the PRINCIPAL honest.
pub fn identity_write_context(claims: Option<&Value>) -> Result<SessionContext, HttpJson<Value>> {
    let context = identity_context(claims)?;
    if !can_write_receivables(&context) {
        return Err(HttpJson::new(
            403,
            json!({
                "code": "insufficient_permission",
                "message": "The authenticated identity cannot modify accounts receivable"
            }),
        ));
    }
    Ok(context)
}

fn can_write_receivables(context: &SessionContext) -> bool {
    context
        .permissions
        .iter()
        .any(|permission| permission == "accounting:invoice:write")
}

pub fn with_ar_transaction<T>(
    context: &SessionContext,
    operation: impl FnOnce(&ExclusivePrimaryLifeExecutor<'_>) -> Result<T, ArError>,
) -> Result<T, ArError> {
    with_ar_isolated(context, lifeguard::IsolationLevel::ReadCommitted, operation)
}

/// One-SNAPSHOT read transaction for multi-statement aggregates.
///
/// Reports such as the open-items list and the aging summary compose several
/// queries (documents, credit notes, overlays, applications, events). Under
/// plain READ COMMITTED each statement sees later commits, so a concurrent
/// posting/settlement can tear the aggregate (a document missing from one leg
/// while its payment counts in another). Every leg therefore runs inside one
/// REPEATABLE READ transaction — the isolation level rides on the `BEGIN`
/// because the pipelined RLS context call is already the first query.
pub fn with_ar_snapshot_transaction<T>(
    context: &SessionContext,
    operation: impl FnOnce(&ExclusivePrimaryLifeExecutor<'_>) -> Result<T, ArError>,
) -> Result<T, ArError> {
    with_ar_isolated(
        context,
        lifeguard::IsolationLevel::RepeatableRead,
        operation,
    )
}

fn with_ar_isolated<T>(
    context: &SessionContext,
    isolation: lifeguard::IsolationLevel,
    operation: impl FnOnce(&ExclusivePrimaryLifeExecutor<'_>) -> Result<T, ArError>,
) -> Result<T, ArError> {
    let database = crate::database::db().map_err(ArError::Database)?;
    let mut domain_error = None;
    let mut output = None;
    let transaction =
        database
            .pool()
            .with_session_transaction_isolated(context, isolation, |executor| {
                match operation(executor) {
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
        ArError::NotFound => (404, "NOT_FOUND", "Resource not found".to_string()),
        ArError::Conflict(_) => (409, "CONFLICT", error.to_string()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use lifeguard::SessionContext;
    use uuid::Uuid;

    fn context(permissions: &[&str]) -> SessionContext {
        SessionContext {
            tenant_id: "hauliage".to_string(),
            subject_id: Uuid::nil(),
            organization_id: Uuid::nil(),
            session_id: "session-1".to_string(),
            roles: vec!["billing".to_string()],
            permissions: permissions.iter().map(|p| (*p).to_string()).collect(),
            user_type: Some("service".to_string()),
            org_type: Some("shipper".to_string()),
        }
    }

    #[test]
    fn write_permission_is_exact_and_separate_from_read() {
        // GL precedent: a read-capable principal cannot mutate receivables.
        assert!(can_write_receivables(&context(&[
            "accounting:invoice:write"
        ])));
        assert!(!can_write_receivables(&context(&[
            "accounting:ledger:read"
        ])));
        assert!(can_write_receivables(&context(&[
            "accounting:ledger:read",
            "accounting:invoice:write"
        ])));
        assert!(!can_write_receivables(&context(&[])));
    }

    #[test]
    fn claims_without_write_permission_are_rejected() {
        let claims = serde_json::json!({
            "sub": "a1000001-0001-4000-8000-000000000004",
            "user_id": "a1000001-0001-4000-8000-000000000004",
            "sid": "session-1",
            "tenant_id": "hauliage",
            "org_id": "b2000002-0002-4000-8000-000000000002",
            "user_type": "service",
            "https://sesame-idam.dev/claims": {
                "tenant": "hauliage",
                "roles": ["billing"],
                "permissions": ["accounting:ledger:read"]
            }
        });
        assert!(identity_write_context(Some(&claims)).is_err());
        assert!(identity_write_context(None).is_err());
    }
}
