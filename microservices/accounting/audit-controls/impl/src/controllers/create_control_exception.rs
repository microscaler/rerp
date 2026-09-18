// Implementation stub for handler 'create_control_exception'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_audit_controls_gen::handlers::create_control_exception::{Request, Response};

#[handler(CreateControlExceptionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        entity_id: "".to_string(),
        id: "".to_string(),
        reason: "".to_string(),
        service_name: "".to_string(),
        status: serde_json::json!({}),
    }
}
