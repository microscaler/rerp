// User-owned controller for handler 'list_events'.

use crate::handlers::list_events::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::MessageEvent;

#[handler(ListEventsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "at": "2026-11-05T16:00:00Z",
    //       "contact_id": "c891a339-996d-56b9-a022-244c570f48c9",
    //       "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
    //       "send_id": "a28a1db3-cee2-594e-adbd-25b5f5f34365",
    //       "type": "QUEUED"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "at": "2026-11-05T16:00:00Z",
      "contact_id": "c891a339-996d-56b9-a022-244c570f48c9",
      "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
      "send_id": "a28a1db3-cee2-594e-adbd-25b5f5f34365",
      "type": "QUEUED"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 2
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<MessageEvent>(serde_json::json!({"at":"2026-11-05T16:00:00Z","contact_id":"c891a339-996d-56b9-a022-244c570f48c9","id":"3eb009b7-a99d-5a4c-956a-99e11d8dbb76","send_id":"a28a1db3-cee2-594e-adbd-25b5f5f34365","type":"QUEUED"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
