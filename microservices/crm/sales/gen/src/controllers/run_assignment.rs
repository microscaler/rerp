// User-owned controller for handler 'run_assignment'.

use crate::handlers::run_assignment::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RunAssignmentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        assigned_count: 42,
        details: vec![],
        failed_count: 42,
    })
}
