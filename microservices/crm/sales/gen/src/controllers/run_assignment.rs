// User-owned controller for handler 'run_assignment'.

use crate::handlers::run_assignment::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RunAssignmentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    // Example response:
    // {
    //   "assigned_count": 3,
    //   "details": [
    //     {}
    //   ],
    //   "failed_count": 3
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "assigned_count": 3,
  "details": [
    {}
  ],
  "failed_count": 3
}"###,
    ) {
        Ok(parsed) => return ApiResponse::Ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    ApiResponse::Ok(Response {
        assigned_count: 3,
        details: vec![
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)
        ],
        failed_count: 3,
    })
}
