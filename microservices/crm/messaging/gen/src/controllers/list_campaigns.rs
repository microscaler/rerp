// User-owned controller for handler 'list_campaigns'.

use crate::handlers::list_campaigns::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Campaign;

#[handler(ListCampaignsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "channel": "EMAIL",
    //       "created_at": "2026-11-05T16:00:00Z",
    //       "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
    //       "kind": "ONE_TIME",
    //       "list_ids": [
    //         "d9110aaa-756c-5aae-baf7-d708ef765e10"
    //       ],
    //       "name": "Northwind Traders",
    //       "status": "DRAFT"
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
      "channel": "EMAIL",
      "created_at": "2026-11-05T16:00:00Z",
      "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
      "kind": "ONE_TIME",
      "list_ids": [
        "d9110aaa-756c-5aae-baf7-d708ef765e10"
      ],
      "name": "Northwind Traders",
      "status": "DRAFT"
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
        items: vec![serde_json::from_value::<Campaign>(serde_json::json!({"channel":"EMAIL","created_at":"2026-11-05T16:00:00Z","id":"3eb009b7-a99d-5a4c-956a-99e11d8dbb76","kind":"ONE_TIME","list_ids":["d9110aaa-756c-5aae-baf7-d708ef765e10"],"name":"Northwind Traders","status":"DRAFT"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
