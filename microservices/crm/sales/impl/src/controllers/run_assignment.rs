// Implementation stub for handler 'run_assignment'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_crm_sales_gen::handlers::run_assignment::{ApiResponse, Request, Response};

#[handler(RunAssignmentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let idempotency_key = req.inner.idempotency_key;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    ApiResponse::Ok(Response {
        assigned_count: 3, // TODO: Set from your business logic
        details: vec![
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)
        ], // TODO: Set from your business logic
        failed_count: 3,   // TODO: Set from your business logic
    })
}
