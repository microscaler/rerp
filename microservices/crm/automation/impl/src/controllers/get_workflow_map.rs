// Implementation stub for handler 'get_workflow_map'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_automation_gen::handlers::get_workflow_map::{Request, Response};

#[handler(GetWorkflowMapController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        active: 1204, // TODO: Set from your business logic
        completed: 8311, // TODO: Set from your business logic
        entered_total: None, // TODO: Set from your business logic
        failed: None, // TODO: Set from your business logic
        goal_reached: None, // TODO: Set from your business logic
        steps: vec![serde_json::from_str::<serde_json::Value>(r#"{"branch_no":0,"branch_yes":0,"clicked":1142,"detail":"Welcome · day 0: what you can do today","failed":2,"kind":"ACTION","label":"Send email","opened":5899,"passed":9515,"sent":9515,"step_id":"f1189b89-3e27-5826-bc93-e33f7450d394","waiting_now":0}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"branch_no":0,"branch_yes":0,"clicked":0,"detail":"","failed":0,"kind":"WAIT","label":"Wait 2 days","opened":0,"passed":9513,"sent":0,"step_id":"6987f07d-c099-529a-bb28-e900ce5e5d00","waiting_now":402}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"branch_no":3471,"branch_yes":5640,"clicked":0,"detail":"engagement:opened:2","failed":0,"kind":"IF_ELSE","label":"Opened the previous email?","opened":0,"passed":9111,"sent":0,"step_id":"94f0ecb3-8d57-54c0-937b-10c11b11cb2c","waiting_now":0}"#).unwrap_or(serde_json::Value::Null)], // TODO: Set from your business logic
        stopped: None, // TODO: Set from your business logic
        workflow_id: "22e49987-c27c-538a-9688-56d9ca17a7ec".to_string(), // TODO: Set from your business logic
    })
}
