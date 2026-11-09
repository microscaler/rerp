// User-owned controller for handler 'fire_trigger'.

use crate::handlers::fire_trigger::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(FireTriggerController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        executed: Some(true),
        workflows_fired: Some(42),
    })
}
