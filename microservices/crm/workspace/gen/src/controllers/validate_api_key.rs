// User-owned controller for handler 'validate_api_key'.

use crate::handlers::validate_api_key::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ValidateApiKeyController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    // Example response:
    // {
    //   "rate_limit_remaining": 3,
    //   "scope": {},
    //   "valid": false
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "rate_limit_remaining": 3,
  "scope": {},
  "valid": false
}"###,
    ) {
        Ok(parsed) => return ApiResponse::Ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    ApiResponse::Ok(Response {
        rate_limit_remaining: Some(3),
        scope: Some(
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),
        ),
        valid: Some(false),
    })
}
