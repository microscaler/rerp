// Implementation stub for handler 'list_data_requests'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_workspace_gen::handlers::list_data_requests::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_workspace_gen::handlers::types::DataRequest;

#[handler(ListDataRequestsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let status = req.inner.status;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<DataRequest>(serde_json::json!({"contact_email":"marcus.reid@adventure.example","created_at":"2026-11-02T16:00:00Z","id":"c4faeb88-b400-5f86-84ae-47f52514f082","kind":"ACCESS","note":"2.1 MB JSON","requested_by":"Contact · preference page","status":"DONE"})).unwrap_or_default(), serde_json::from_value::<DataRequest>(serde_json::json!({"contact_email":"old-team@fabrikam.example","created_at":"2026-09-30T16:00:00Z","id":"bac118f7-c5eb-539d-bde0-41b5b513bce3","kind":"ACCESS","note":"Anonymised · consent log kept","requested_by":"Alex Morgan · ticket #4412","status":"DONE"})).unwrap_or_default(), serde_json::from_value::<DataRequest>(serde_json::json!({"contact_email":"j.doe@example.org","created_at":"2026-11-04T16:00:00Z","id":"8b40f48a-b29b-5329-a1e1-39f7966192c4","kind":"ACCESS","note":"28 days left","requested_by":"Contact · email to privacy@","status":"PENDING_REVIEW"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 3, // TODO: Set from your business logic
    })
}
