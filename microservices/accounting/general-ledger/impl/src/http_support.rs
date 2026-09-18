//! Shared HTTP adaptation for Sesame identity and Lifeguard transactions.

use brrtrouter::typed::HttpJson;
use lifeguard::{ExclusivePrimaryLifeExecutor, LifeError, SessionContext};
use serde_json::{json, Value};

use crate::ledger::LedgerError;

pub fn identity_context(claims: Option<&Value>) -> Result<SessionContext, HttpJson<Value>> {
    let context = crate::identity::from_validated_claims(claims).map_err(|error| {
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
    })?;
    if !can_read_ledger(&context) {
        return Err(HttpJson::new(
            403,
            json!({
                "code": "insufficient_permission",
                "message": "The authenticated identity cannot read the General Ledger"
            }),
        ));
    }
    Ok(context)
}

fn can_read_ledger(context: &SessionContext) -> bool {
    context
        .permissions
        .iter()
        .any(|permission| permission == "accounting:ledger:read")
}

pub fn with_accounting_transaction<T>(
    context: &SessionContext,
    operation: impl FnOnce(&ExclusivePrimaryLifeExecutor<'_>) -> Result<T, LedgerError>,
) -> Result<T, LedgerError> {
    let database = crate::database::db().map_err(LedgerError::Database)?;
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
                        "ledger query rejected; transaction rolled back".to_string(),
                    ))
                }
            });
    match (transaction, domain_error, output) {
        (Ok(()), None, Some(value)) => Ok(value),
        (Ok(()), None, None) => Err(LedgerError::Database(
            "transaction completed without a ledger result".to_string(),
        )),
        (Err(error), None, _) => Err(LedgerError::Database(error.to_string())),
        (_, Some(error), _) => Err(error),
    }
}

pub fn ledger_error(error: LedgerError) -> HttpJson<Value> {
    let (status, code, message) = match &error {
        LedgerError::Validation(_) => (400, "validation_error", error.to_string()),
        LedgerError::Policy(_) => (403, "accounting_scope_unavailable", error.to_string()),
        LedgerError::NotFound => (
            404,
            "not_found",
            "The ledger resource was not found".to_string(),
        ),
        LedgerError::Database(_) => {
            eprintln!("General Ledger database failure: {error}");
            (
                500,
                "accounting_persistence_error",
                "The ledger query could not be completed".to_string(),
            )
        }
    };
    HttpJson::new(status, json!({ "code": code, "message": message }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn context(permissions: &[&str]) -> SessionContext {
        SessionContext {
            tenant_id: "hauliage".to_string(),
            subject_id: Uuid::new_v4(),
            organization_id: Uuid::new_v4(),
            session_id: "session-1".to_string(),
            roles: vec!["finance-reader".to_string()],
            permissions: permissions
                .iter()
                .map(|permission| (*permission).to_string())
                .collect(),
            user_type: Some("service".to_string()),
            org_type: None,
        }
    }

    #[test]
    fn requires_exact_ledger_read_permission() {
        assert!(can_read_ledger(&context(&["accounting:ledger:read"])));
        assert!(!can_read_ledger(&context(&["accounting:invoice:write"])));
        assert!(!can_read_ledger(&context(&[])));
    }
}
