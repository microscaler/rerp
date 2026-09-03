// BRRTRouter: user-owned
//! Claim-based authorization for crm-pipeline (US_31_04, #135).
//!
//! BRRTRouter's jwks provider verifies signature/exp/iss/aud before a
//! request reaches a handler, but verification alone admits ANY active
//! sesame tenant (hauliage users included). These guards add the
//! authorization layer on top of it:
//!
//! * every operation requires tenant `pricewhisperer`;
//! * reads require `crm.viewer` or `crm.editor`;
//! * mutations (PUT /leads/{id}, PATCH /leads/{id}/stage) require
//!   `crm.editor`.
//!
//! Token shape (decoded from a live sesame access token, 2026-09-03):
//! top-level `tenant_id: "pricewhisperer"`, and a namespaced object
//! `"https://sesame-idam.dev/claims"` carrying `tenant`, `portal`,
//! `roles: ["crm.editor", ...]` and `permissions`. Roles come from
//! sesame `role_assignments` via authz-core.

use brrtrouter::typed::HttpJson;
use serde_json::{json, Value};

const CLAIMS_NAMESPACE: &str = "https://sesame-idam.dev/claims";
const REQUIRED_TENANT: &str = "pricewhisperer";
const ROLE_EDITOR: &str = "crm.editor";
const ROLE_VIEWER: &str = "crm.viewer";

fn deny(status: u16, error: &str, message: &str) -> HttpJson<Value> {
    HttpJson::new(
        status,
        json!({ "code": status, "error": error, "message": message }),
    )
}

fn namespaced(claims: &Value) -> Option<&Value> {
    claims.get(CLAIMS_NAMESPACE)
}

/// Both the top-level `tenant_id` and the namespaced `tenant` are issued
/// by sesame; accept only when at least one names our tenant and neither
/// names a different one.
fn tenant_matches(claims: &Value) -> bool {
    let top = claims.get("tenant_id").and_then(Value::as_str);
    let ns = namespaced(claims)
        .and_then(|c| c.get("tenant"))
        .and_then(Value::as_str);
    match (top, ns) {
        (Some(t), _) if t != REQUIRED_TENANT => false,
        (_, Some(t)) if t != REQUIRED_TENANT => false,
        (None, None) => false,
        _ => true,
    }
}

fn has_role(claims: &Value, wanted: &str) -> bool {
    namespaced(claims)
        .and_then(|c| c.get("roles"))
        .and_then(Value::as_array)
        .is_some_and(|roles| roles.iter().any(|r| r.as_str() == Some(wanted)))
}

fn require(claims: Option<&Value>, roles: &[&str]) -> Result<(), HttpJson<Value>> {
    let Some(claims) = claims else {
        // The jwks provider rejects unauthenticated requests before
        // dispatch; reaching here without claims means the route was
        // served without bearer verification - fail closed.
        return Err(deny(401, "unauthenticated", "missing verified token claims"));
    };
    if !tenant_matches(claims) {
        return Err(deny(403, "wrong_tenant", "token tenant is not permitted for this API"));
    }
    if !roles.iter().any(|role| has_role(claims, role)) {
        return Err(deny(403, "missing_role", "token lacks a CRM role for this operation"));
    }
    Ok(())
}

/// Read access: `crm.viewer` or `crm.editor`, tenant `pricewhisperer`.
pub fn require_viewer(claims: Option<&Value>) -> Result<(), HttpJson<Value>> {
    require(claims, &[ROLE_VIEWER, ROLE_EDITOR])
}

/// Mutation access: `crm.editor` only, tenant `pricewhisperer`.
pub fn require_editor(claims: Option<&Value>) -> Result<(), HttpJson<Value>> {
    require(claims, &[ROLE_EDITOR])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token(tenant: &str, roles: &[&str]) -> Value {
        json!({
            "tenant_id": tenant,
            CLAIMS_NAMESPACE: { "tenant": tenant, "roles": roles, "permissions": [] }
        })
    }

    #[test]
    fn editor_passes_both_gates() {
        let claims = token("pricewhisperer", &["crm.editor"]);
        assert!(require_viewer(Some(&claims)).is_ok());
        assert!(require_editor(Some(&claims)).is_ok());
    }

    #[test]
    fn viewer_reads_but_cannot_mutate() {
        let claims = token("pricewhisperer", &["crm.viewer"]);
        assert!(require_viewer(Some(&claims)).is_ok());
        assert!(require_editor(Some(&claims)).is_err());
    }

    #[test]
    fn other_tenant_is_rejected_even_with_role() {
        let claims = token("hauliage", &["crm.editor"]);
        assert!(require_viewer(Some(&claims)).is_err());
        assert!(require_editor(Some(&claims)).is_err());
    }

    #[test]
    fn missing_roles_rejected() {
        let claims = token("pricewhisperer", &[]);
        assert!(require_viewer(Some(&claims)).is_err());
    }

    #[test]
    fn missing_claims_rejected() {
        assert!(require_viewer(None).is_err());
        assert!(require_editor(None).is_err());
    }

    #[test]
    fn tenant_only_top_level_still_passes() {
        let claims = json!({
            "tenant_id": "pricewhisperer",
            CLAIMS_NAMESPACE: { "roles": ["crm.viewer"] }
        });
        assert!(require_viewer(Some(&claims)).is_ok());
    }
}
