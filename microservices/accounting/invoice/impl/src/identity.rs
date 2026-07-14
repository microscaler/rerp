//! Convert BRRTRouter-validated Sesame claims into Lifeguard's RLS context.

use lifeguard::SessionContext;
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentityError {
    MissingValidatedClaims,
    MissingField(&'static str),
    InvalidField(&'static str),
    ClaimMismatch {
        first: &'static str,
        second: &'static str,
    },
}

impl std::fmt::Display for IdentityError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingValidatedClaims => formatter.write_str("validated claims are missing"),
            Self::MissingField(field) => write!(formatter, "required claim is missing: {field}"),
            Self::InvalidField(field) => {
                write!(formatter, "claim has an invalid type or value: {field}")
            }
            Self::ClaimMismatch { first, second } => {
                write!(formatter, "validated claims disagree: {first} and {second}")
            }
        }
    }
}

impl std::error::Error for IdentityError {}

fn required_string<'a>(value: &'a Value, field: &'static str) -> Result<&'a str, IdentityError> {
    let string = value
        .as_str()
        .ok_or(IdentityError::InvalidField(field))?
        .trim();
    if string.is_empty() {
        return Err(IdentityError::InvalidField(field));
    }
    Ok(string)
}

fn required_uuid(value: &Value, field: &'static str) -> Result<Uuid, IdentityError> {
    Uuid::parse_str(required_string(value, field)?).map_err(|_| IdentityError::InvalidField(field))
}

fn string_array(value: &Value, field: &'static str) -> Result<Vec<String>, IdentityError> {
    value
        .as_array()
        .ok_or(IdentityError::InvalidField(field))?
        .iter()
        .map(|entry| required_string(entry, field).map(str::to_string))
        .collect()
}

pub fn from_validated_claims(claims: Option<&Value>) -> Result<SessionContext, IdentityError> {
    let claims = claims.ok_or(IdentityError::MissingValidatedClaims)?;
    let tenant = required_string(
        claims
            .get("tenant_id")
            .ok_or(IdentityError::MissingField("tenant_id"))?,
        "tenant_id",
    )?;
    let subject_id = required_uuid(
        claims
            .get("sub")
            .ok_or(IdentityError::MissingField("sub"))?,
        "sub",
    )?;
    let user_id = required_uuid(
        claims
            .get("user_id")
            .ok_or(IdentityError::MissingField("user_id"))?,
        "user_id",
    )?;
    if subject_id != user_id {
        return Err(IdentityError::ClaimMismatch {
            first: "sub",
            second: "user_id",
        });
    }
    let organization_id = required_uuid(
        claims
            .get("org_id")
            .ok_or(IdentityError::MissingField("org_id"))?,
        "org_id",
    )?;
    let session_id = required_string(
        claims
            .get("sid")
            .ok_or(IdentityError::MissingField("sid"))?,
        "sid",
    )?;

    let authorization = claims
        .get("https://sesame-idam.dev/claims")
        .and_then(Value::as_object)
        .ok_or(IdentityError::MissingField(
            "https://sesame-idam.dev/claims",
        ))?;
    let authorization_tenant = required_string(
        authorization
            .get("tenant")
            .ok_or(IdentityError::MissingField("sx.tenant"))?,
        "sx.tenant",
    )?;
    if tenant != authorization_tenant {
        return Err(IdentityError::ClaimMismatch {
            first: "tenant_id",
            second: "sx.tenant",
        });
    }

    let roles = string_array(
        authorization
            .get("roles")
            .ok_or(IdentityError::MissingField("sx.roles"))?,
        "sx.roles",
    )?;
    let permissions = string_array(
        authorization
            .get("permissions")
            .ok_or(IdentityError::MissingField("sx.permissions"))?,
        "sx.permissions",
    )?;
    let user_type = claims
        .get("user_type")
        .map(|value| required_string(value, "user_type").map(str::to_string))
        .transpose()?;
    let org_type = authorization
        .get("org_type")
        .map(|value| required_string(value, "sx.org_type").map(str::to_string))
        .transpose()?;

    Ok(SessionContext {
        tenant_id: tenant.to_string(),
        subject_id,
        organization_id,
        session_id: session_id.to_string(),
        roles,
        permissions,
        user_type,
        org_type,
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
