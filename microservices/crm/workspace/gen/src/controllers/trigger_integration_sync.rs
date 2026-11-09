// User-owned controller for handler 'trigger_integration_sync'.

use crate::handlers::trigger_integration_sync::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(TriggerIntegrationSyncController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    // Example response:
    // {
    //   "status": "Status",
    //   "sync_id": "827087d0-d598-5430-b72d-ea37c0dfc1b1"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "status": "Status",
  "sync_id": "827087d0-d598-5430-b72d-ea37c0dfc1b1"
}"###,
    ) {
        Ok(parsed) => return ApiResponse::Ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    ApiResponse::Ok(Response {
        status: Some("Status".to_string()),
        sync_id: Some("827087d0-d598-5430-b72d-ea37c0dfc1b1".to_string()),
    })
}
