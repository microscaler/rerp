// User-owned controller for handler 'list_tracked_sites'.

use crate::handlers::list_tracked_sites::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::TrackedSite;

#[handler(ListTrackedSitesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "created_at": "2026-11-05T16:00:00Z",
    //       "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
    //       "origin": "https://northwindtraders.example/pricing",
    //       "tracking_key": "Tracking key"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "created_at": "2026-11-05T16:00:00Z",
      "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
      "origin": "https://northwindtraders.example/pricing",
      "tracking_key": "Tracking key"
    }
  ]
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response(vec![serde_json::from_value::<TrackedSite>(serde_json::json!({"created_at":"2026-11-05T16:00:00Z","id":"e67555dd-33f4-5fee-9da3-a019895219b1","origin":"https://northwindtraders.example/pricing","tracking_key":"Tracking key"})).unwrap_or_default()]))
}
