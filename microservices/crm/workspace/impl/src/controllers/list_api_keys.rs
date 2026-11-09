// Implementation stub for handler 'list_api_keys'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_workspace_gen::handlers::list_api_keys::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_workspace_gen::handlers::types::ApiKey;

#[handler(ListApiKeysController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<ApiKey>(serde_json::json!({"created_at":"2026-07-28T16:00:00Z","id":"b90438d1-e134-5543-919f-19864d0919d3","is_active":true,"last_used_at":"2026-11-05T15:59:48Z","name":"Website (public tracking key)","scopes":{}})).unwrap_or_default(), serde_json::from_value::<ApiKey>(serde_json::json!({"created_at":"2026-07-28T16:00:00Z","id":"08dd68b1-5f07-5687-b207-d5b9cf18daaf","is_active":true,"last_used_at":"2026-11-05T15:59:59Z","name":"Product backend","scopes":{}})).unwrap_or_default(), serde_json::from_value::<ApiKey>(serde_json::json!({"created_at":"2026-07-28T16:00:00Z","id":"02a937b8-4dcd-59b0-9797-dcefad332287","is_active":true,"last_used_at":"2026-11-04T16:00:00Z","name":"BI export","scopes":{}})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 3, // TODO: Set from your business logic
    })
}
