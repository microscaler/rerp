// User-owned controller for handler 'test_webhook'.

use crate::handlers::test_webhook::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(TestWebhookController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    // Example response:
    // {}
    match serde_json::from_str::<Response>(r###"{}"###) {
        Ok(parsed) => return ApiResponse::Ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    ApiResponse::Ok(Response {
        response_code: Some(42),
        sent: Some(true),
    })
}
