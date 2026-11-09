// User-owned controller for handler 'list_jobs'.

use crate::handlers::list_jobs::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Job;

#[handler(ListJobsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "entity": "CONTACTS",
    //       "id": "94b293ed-258b-5091-8e79-75d2ac471f25",
    //       "kind": "IMPORT",
    //       "queued_at": "2026-11-05T16:00:00Z",
    //       "requested_by": "Requested by",
    //       "status": "QUEUED"
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
      "entity": "CONTACTS",
      "id": "94b293ed-258b-5091-8e79-75d2ac471f25",
      "kind": "IMPORT",
      "queued_at": "2026-11-05T16:00:00Z",
      "requested_by": "Requested by",
      "status": "QUEUED"
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
        items: vec![serde_json::from_value::<Job>(serde_json::json!({"entity":"CONTACTS","id":"94b293ed-258b-5091-8e79-75d2ac471f25","kind":"IMPORT","queued_at":"2026-11-05T16:00:00Z","requested_by":"Requested by","status":"QUEUED"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
