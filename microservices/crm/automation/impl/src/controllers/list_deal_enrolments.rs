// Implementation stub for handler 'list_deal_enrolments'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_automation_gen::handlers::list_deal_enrolments::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_automation_gen::handlers::types::Enrolment;

#[handler(ListDealEnrolmentsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let id = req.inner.id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Enrolment>(serde_json::json!({"contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","entered_at":"2026-11-01T16:00:00Z","id":"47ac623d-96a0-57aa-b8c2-0b8a230023a6","status":"COMPLETED","workflow_id":"f1f1bd07-238b-520a-b007-7846dac00c19","workflow_name":"Hot lead handoff"})).unwrap_or_default(), serde_json::from_value::<Enrolment>(serde_json::json!({"contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","current_step_label":"14 days no activity","entered_at":"2026-11-02T16:00:00Z","id":"3b209ba1-4c06-5235-a6e7-0c74f4d40366","status":"WAITING","workflow_id":"b36c0bc7-7dae-56b4-af7f-9c1ef4673496","workflow_name":"Stalled deal nudge"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 2, // TODO: Set from your business logic
    })
}
