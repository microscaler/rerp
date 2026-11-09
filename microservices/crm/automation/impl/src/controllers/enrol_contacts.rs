// Implementation stub for handler 'enrol_contacts'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_automation_gen::handlers::enrol_contacts::{Request, Response};

#[handler(EnrolContactsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let contact_ids = req.inner.contact_ids;// let start_step_id = req.inner.start_step_id;// let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        already_active: 3,   // TODO: Set from your business logic
        enrolled: 0,         // TODO: Set from your business logic
        enrolment_ids: None, // TODO: Set from your business logic
        excluded: 3,         // TODO: Set from your business logic
        skipped: None,       // TODO: Set from your business logic
        workflow_id: None,   // TODO: Set from your business logic
    })
}
