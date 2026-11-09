// User-owned controller for handler 'capture_social_lead'.

use crate::handlers::capture_social_lead::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CaptureSocialLeadController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "lead_id": "8c42f135-110c-57a8-bfa6-c01026615082",
    //   "matched_existing": false
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "lead_id": "8c42f135-110c-57a8-bfa6-c01026615082",
  "matched_existing": false
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        lead_id: Some("8c42f135-110c-57a8-bfa6-c01026615082".to_string()),
        matched_existing: Some(false),
    })
}
