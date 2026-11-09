// User-owned controller for handler 'sync_ad_audience'.

use crate::handlers::sync_ad_audience::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(SyncAdAudienceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "audience_id": "50b1012e-f304-5512-8265-bfc8e7df9a55",
    //   "id": "b6d841b8-e819-50bb-ba55-f9f0c0e679bf",
    //   "status": "QUEUED"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "audience_id": "50b1012e-f304-5512-8265-bfc8e7df9a55",
  "id": "b6d841b8-e819-50bb-ba55-f9f0c0e679bf",
  "status": "QUEUED"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        added: Some(42),
        audience_id: "50b1012e-f304-5512-8265-bfc8e7df9a55".to_string(),
        error: Some("example".to_string()),
        failed: Some(42),
        finished_at: Some("example".to_string()),
        id: "b6d841b8-e819-50bb-ba55-f9f0c0e679bf".to_string(),
        platform_size: Some(42),
        removed: Some(42),
        started_at: Some("example".to_string()),
        status: "QUEUED".to_string(),
        trigger: Some("example".to_string()),
        uploaded_total: Some(42),
    })
}
