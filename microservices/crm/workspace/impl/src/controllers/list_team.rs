// Implementation stub for handler 'list_team'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_workspace_gen::handlers::list_team::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_workspace_gen::handlers::types::TeamMember;

#[handler(ListTeamController)]
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
        items: vec![serde_json::from_value::<TeamMember>(serde_json::json!({"can_approve_social":true,"can_send":true,"can_send_label":"Yes","last_active_at":"2026-11-05T16:00:00Z","name":"Alex Morgan","role":"owner","user_id":"alex"})).unwrap_or_default(), serde_json::from_value::<TeamMember>(serde_json::json!({"can_approve_social":false,"can_send":false,"can_send_label":"No · transactional only","last_active_at":"2026-11-05T14:00:00Z","name":"Sam Lee","role":"owner","user_id":"sam"})).unwrap_or_default(), serde_json::from_value::<TeamMember>(serde_json::json!({"can_approve_social":true,"can_send":false,"can_send_label":"Drafts only","last_active_at":"2026-11-04T16:00:00Z","name":"Riya Kapoor","role":"owner","user_id":"riya"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 2, // TODO: Set from your business logic
    })
}
