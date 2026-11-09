// User-owned controller for handler 'validate_api_key'.

use crate::handlers::validate_api_key::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ValidateApiKeyController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        rate_limit_remaining: Some(42),
        scope: Some(Default::default()),
        valid: Some(true),
    })
}
