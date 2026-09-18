//! Convert BRRTRouter-validated Sesame claims into Lifeguard's RLS context.

use lifeguard::SessionContext;
use serde_json::Value;
pub use sesame_idam_client::ClaimsError as IdentityError;

pub fn from_validated_claims(claims: Option<&Value>) -> Result<SessionContext, IdentityError> {
    let claims = sesame_idam_client::parse_validated_claims(claims)?;

    Ok(SessionContext {
        tenant_id: claims.tenant_id,
        subject_id: claims.subject_id,
        organization_id: claims.organization_id,
        session_id: claims.session_id,
        roles: claims.roles,
        permissions: claims.permissions,
        user_type: claims.user_type,
        org_type: claims.org_type,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn claims() -> Value {
        serde_json::json!({
            "sub": "a1000001-0001-4000-8000-000000000004",
            "user_id": "a1000001-0001-4000-8000-000000000004",
            "sid": "session-1",
            "tenant_id": "hauliage",
            "org_id": "b2000002-0002-4000-8000-000000000002",
            "user_type": "service",
            "https://sesame-idam.dev/claims": {
                "tenant": "hauliage",
                "roles": ["billing"],
                "permissions": ["accounting:invoice:write"]
            }
        })
    }

    #[test]
    fn builds_complete_context() {
        let context = from_validated_claims(Some(&claims())).expect("valid claims");
        assert_eq!(context.tenant_id, "hauliage");
        assert_eq!(context.roles, ["billing"]);
        assert_eq!(context.permissions, ["accounting:invoice:write"]);
    }

    #[test]
    fn rejects_tenant_mismatch() {
        let mut value = claims();
        value["https://sesame-idam.dev/claims"]["tenant"] =
            Value::String("another-tenant".to_string());
        assert!(matches!(
            from_validated_claims(Some(&value)),
            Err(IdentityError::ClaimMismatch { .. })
        ));
    }

    #[test]
    fn rejects_unvalidated_request() {
        assert_eq!(
            from_validated_claims(None),
            Err(IdentityError::MissingValidatedClaims)
        );
    }
}
