// Implementation stub for handler 'get_automation_goals'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_insights_gen::handlers::get_automation_goals::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_insights_gen::handlers::types::AutomationGoalRow;

#[handler(GetAutomationGoalsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;// let from = req.inner.from;// let to = req.inner.to;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        goals: vec![serde_json::from_value::<AutomationGoalRow>(serde_json::json!({"achieved_rate":0.23,"avg_seconds_to_achieve":207_360,"condition":"Site event trial_activated occurred","contacts_achieved":377,"contacts_skipped":1025,"contacts_waiting":238,"name":"Activated the product","step_id":"047a18d8-3dbe-5b48-8733-4d69800e8319"})).unwrap_or_default()], // TODO: Set from your business logic
        workflow_id: "22e49987-c27c-538a-9688-56d9ca17a7ec".to_string(), // TODO: Set from your business logic
    })
}
