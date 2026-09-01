// User-owned controller for handler 'detect_duplicates'.

use crate::handlers::detect_duplicates::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DetectDuplicatesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response { duplicates: vec![] })
}
