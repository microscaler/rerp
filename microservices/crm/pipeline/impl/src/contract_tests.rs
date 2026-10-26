//! Contract tests: bodies this service builds by hand (`HttpJson<Value>`), checked against the
//! response schemas of the spec it serves (`gen/doc/openapi.yaml`) the way the server's response
//! validation checks them (`brrtrouter::contract`).

use brrtrouter::contract::{ContractError, ResponseContract};
use brrtrouter::typed::TypedHandlerRequest;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::OnceLock;

fn contract() -> &'static ResponseContract {
    static C: OnceLock<ResponseContract> = OnceLock::new();
    C.get_or_init(|| {
        ResponseContract::load(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../gen/doc/openapi.yaml"
        ))
        .expect("crm pipeline spec")
    })
}

fn claims(tenant: &str, roles: &[&str]) -> Value {
    json!({
        "tenant_id": tenant,
        "https://sesame-idam.dev/claims": { "tenant": tenant, "roles": roles, "permissions": [] }
    })
}

/// GET /stages builds its page from the static stage list.
#[test]
fn list_stages_matches_its_schema() {
    let req = TypedHandlerRequest {
        method: http::Method::GET,
        path: "/stages".into(),
        handler_name: "list_stages".into(),
        path_params: HashMap::new(),
        query_params: HashMap::new(),
        data: rerp_crm_pipeline_gen::handlers::list_stages::Request {
            page: None,
            limit: None,
            filter_team_id: None,
        },
        jwt_claims: Some(claims("pricewhisperer", &["crm.viewer"])),
    };
    let r = crate::controllers::list_stages::handle(req);
    contract().assert_strict("list_stages", r.status, &r.body);
}

/// The auth gate's refusals (401 without claims, 403 for the wrong tenant or no CRM role), on
/// every operation that declares a schema for that status.
#[test]
fn auth_refusals_match_the_declared_schemas() {
    let refusals = [
        crate::auth::require_viewer(None).unwrap_err(),
        crate::auth::require_viewer(Some(&claims("other", &["crm.viewer"]))).unwrap_err(),
        crate::auth::require_editor(Some(&claims("pricewhisperer", &["crm.viewer"]))).unwrap_err(),
    ];
    let c = contract();
    let (mut checked, mut wrong) = (0, Vec::new());
    for op in c.operations() {
        for r in &refusals {
            match c.check(op, r.status, &r.body) {
                Ok(()) => checked += 1,
                Err(ContractError::NoSchema { .. }) => {}
                Err(e) => wrong.push(e.to_string()),
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    eprintln!("auth refusals checked against {checked} declared schemas");
}
