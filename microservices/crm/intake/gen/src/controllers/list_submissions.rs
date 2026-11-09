// User-owned controller for handler 'list_submissions'.

use crate::handlers::list_submissions::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::IntakeSubmission;

#[handler(ListSubmissionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "id": "19659456-e0be-5b35-8762-ea6182cdf0cb",
    //       "kind": "SUBSCRIPTION",
    //       "status": "PENDING",
    //       "submitted_at": "2026-11-05T16:00:00Z"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "id": "19659456-e0be-5b35-8762-ea6182cdf0cb",
      "kind": "SUBSCRIPTION",
      "status": "PENDING",
      "submitted_at": "2026-11-05T16:00:00Z"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 2
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<IntakeSubmission>(serde_json::json!({"id":"19659456-e0be-5b35-8762-ea6182cdf0cb","kind":"SUBSCRIPTION","status":"PENDING","submitted_at":"2026-11-05T16:00:00Z"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
