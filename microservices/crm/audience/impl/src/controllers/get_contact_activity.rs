// Implementation stub for handler 'get_contact_activity'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_audience_gen::handlers::get_contact_activity::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_audience_gen::handlers::types::Activity;
#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_audience_gen::handlers::types::ActivityKind;

#[handler(GetContactActivityController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;// let page = req.inner.page;// let limit = req.inner.limit;// let kinds = req.inner.kinds;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Activity>(serde_json::json!({"actor":"contact","at":"2026-11-05T14:00:00Z","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","detail":"Campaign · opened twice, clicked Read the changelog","id":"ec537d04-feb3-5edb-8713-78bcb173e477","kind":"MESSAGE_OPENED","summary":"Opened \"October product update\""})).unwrap_or_default(), serde_json::from_value::<Activity>(serde_json::json!({"actor":"contact","at":"2026-11-05T11:00:00Z","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","detail":"From email link · 2 min 40 s · also /docs/sso","id":"17b94f6b-e922-5fd5-8c50-7e7f4a9483f7","kind":"PAGE_VIEW","summary":"Visited /pricing"})).unwrap_or_default(), serde_json::from_value::<Activity>(serde_json::json!({"actor":"contact","at":"2026-11-04T16:00:00Z","contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","detail":"Trigger: date field renewal_date − 30 days","id":"85b3de6c-cd6e-5c99-a045-33d7479d3f12","kind":"WORKFLOW_ENTERED","summary":"Entered \"Renewal reminder · 30 days out\""})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 20, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 10, // TODO: Set from your business logic
    })
}
