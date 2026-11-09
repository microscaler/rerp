// User-owned controller for handler 'detect_duplicates'.

use crate::handlers::detect_duplicates::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DetectDuplicatesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    // Example response:
    // {
    //   "duplicates": [
    //     "6df32aad-3786-5d11-b8d3-f378c2fcef20"
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "duplicates": [
    "6df32aad-3786-5d11-b8d3-f378c2fcef20"
  ]
}"###,
    ) {
        Ok(parsed) => return ApiResponse::Ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    ApiResponse::Ok(Response {
        duplicates: vec!["6df32aad-3786-5d11-b8d3-f378c2fcef20".to_string()],
    })
}
