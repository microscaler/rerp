// Implementation stub for handler 'stop_enrolment'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_automation_gen::handlers::stop_enrolment::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_automation_gen::handlers::types::EnrolmentStatus;
#[allow(unused_imports)]
use rerp_crm_automation_gen::handlers::types::EnrolmentStepRecord;
#[allow(unused_imports)]
use rerp_crm_automation_gen::handlers::types::ExitReason;
#[allow(unused_imports)]
use rerp_crm_automation_gen::handlers::types::TriggerType;

#[handler(StopEnrolmentController)]
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
        contact_email: None, // TODO: Set from your business logic
        contact_id: "af512c4f-0beb-50b6-bf61-7a9d8bd417b5".to_string(), // TODO: Set from your business logic
        contact_name: None,       // TODO: Set from your business logic
        current_step_id: None,    // TODO: Set from your business logic
        current_step_label: None, // TODO: Set from your business logic
        ended_at: None,           // TODO: Set from your business logic
        entered_at: "2026-11-03T16:00:00Z".to_string(), // TODO: Set from your business logic
        entered_by: None,         // TODO: Set from your business logic
        exit_reason: None,        // TODO: Set from your business logic
        goal_step_id: None,       // TODO: Set from your business logic
        history: None,            // TODO: Set from your business logic
        id: "7579e087-fd2f-568e-8835-12a76256ad3e".to_string(), // TODO: Set from your business logic
        next_run_at: None, // TODO: Set from your business logic
        split_paths: None, // TODO: Set from your business logic
        status: serde_json::from_value::<EnrolmentStatus>(serde_json::Value::String(
            "WAITING".to_string(),
        ))
        .unwrap_or_default(), // TODO: Set from your business logic
        trigger_id: None,  // TODO: Set from your business logic
        trigger_subject: None, // TODO: Set from your business logic
        trigger_type: None, // TODO: Set from your business logic
        workflow_id: "22e49987-c27c-538a-9688-56d9ca17a7ec".to_string(), // TODO: Set from your business logic
        workflow_name: None, // TODO: Set from your business logic
    })
}
