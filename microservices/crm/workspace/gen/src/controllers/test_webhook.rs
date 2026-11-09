// User-owned controller for handler 'test_webhook'.

use crate::handlers::test_webhook::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(TestWebhookController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        response_code: Some(42),
        sent: Some(true),
    })
}
