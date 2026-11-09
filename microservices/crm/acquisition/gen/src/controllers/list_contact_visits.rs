// User-owned controller for handler 'list_contact_visits'.

use crate::handlers::list_contact_visits::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::VisitorTracking;

#[handler(ListContactVisitsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "first_seen": "2026-11-05T16:00:00Z",
    //       "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
    //       "visitor_id": "ext_66efa929ba"
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
      "first_seen": "2026-11-05T16:00:00Z",
      "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
      "visitor_id": "ext_66efa929ba"
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
        items: vec![serde_json::from_value::<VisitorTracking>(serde_json::json!({"first_seen":"2026-11-05T16:00:00Z","id":"e67555dd-33f4-5fee-9da3-a019895219b1","visitor_id":"ext_66efa929ba"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
