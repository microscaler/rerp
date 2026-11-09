// User-owned controller for handler 'fire_trigger'.

use crate::handlers::fire_trigger::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(FireTriggerController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    // Example response:
    // {
    //   "executed": false,
    //   "workflows_fired": 3
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "executed": false,
  "workflows_fired": 3
}"###,
    ) {
        Ok(parsed) => return ApiResponse::Ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    ApiResponse::Ok(Response {
        executed: Some(false),
        workflows_fired: Some(3),
    })
}
