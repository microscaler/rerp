// Implementation stub for handler 'list_suppressions'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_audience_gen::handlers::list_suppressions::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_audience_gen::handlers::types::Suppression;

#[handler(ListSuppressionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let search = req.inner.search;// let reason = req.inner.reason;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Suppression>(serde_json::json!({"address":"daniel.kim@wideworld.example","channel":"EMAIL","created_at":"2026-10-28T16:00:00Z","id":"fc58b14d-e13f-5d43-bad7-0a86f5d330c6","note":"October product update","reason":"HARD_BOUNCE","source":"October product update"})).unwrap_or_default(), serde_json::from_value::<Suppression>(serde_json::json!({"address":"old-team@fabrikam.example","channel":"EMAIL","created_at":"2026-09-30T16:00:00Z","id":"a6c4c319-1fb1-5d3c-b7bc-4626e853c8f7","note":"September product update","reason":"COMPLAINT","source":"September product update"})).unwrap_or_default(), serde_json::from_value::<Suppression>(serde_json::json!({"address":"ceo@contoso.example","channel":"EMAIL","created_at":"2026-10-12T16:00:00Z","id":"434e2868-e253-5882-bd99-f063c5c7a6aa","note":"asked by account manager","reason":"MANUAL","source":"Alex Morgan"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 5, // TODO: Set from your business logic
    })
}
