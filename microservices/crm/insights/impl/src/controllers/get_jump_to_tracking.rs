// Implementation stub for handler 'get_jump_to_tracking'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_insights_gen::handlers::get_jump_to_tracking::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_insights_gen::handlers::types::JumpToRow;

#[handler(GetJumpToTrackingController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let workflow_id = req.inner.workflow_id;// let from = req.inner.from;// let to = req.inner.to;// let page = req.inner.page;// let limit = req.inner.limit;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<JumpToRow>(serde_json::json!({"contacts_jumped":377,"contacts_passed":1025,"goal_name":"Activated the product","jump_rate":0.23,"step_id":"047a18d8-3dbe-5b48-8733-4d69800e8319","workflow_id":"22e49987-c27c-538a-9688-56d9ca17a7ec","workflow_name":"Welcome series (7 days)"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 20, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 1, // TODO: Set from your business logic
    })
}
