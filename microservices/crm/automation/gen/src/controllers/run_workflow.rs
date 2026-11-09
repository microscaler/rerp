// User-owned controller for handler 'run_workflow'.

use crate::handlers::run_workflow::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RunWorkflowController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        actions_ran: 42,
        errors: vec![],
        executed: true,
        workflow_id: "example".to_string(),
    })
}
