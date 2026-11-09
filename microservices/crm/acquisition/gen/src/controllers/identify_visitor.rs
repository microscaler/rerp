// User-owned controller for handler 'identify_visitor'.

use crate::handlers::identify_visitor::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(IdentifyVisitorController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    // Example response:
    // {
    //   "identified": false,
    //   "lead_id": "a46e2e3a-01f7-5c22-abcb-22698f556fda",
    //   "visitor_id": "ext_66efa929ba"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "identified": false,
  "lead_id": "a46e2e3a-01f7-5c22-abcb-22698f556fda",
  "visitor_id": "ext_66efa929ba"
}"#,
    ) {
        Ok(parsed) => return ApiResponse::Ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    ApiResponse::Ok(Response {
        identified: Some(false),
        lead_id: Some("a46e2e3a-01f7-5c22-abcb-22698f556fda".to_string()),
        visitor_id: Some("ext_66efa929ba".to_string()),
    })
}
