// User-owned controller for handler 'list_tags'.

use crate::handlers::list_tags::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Tag;

#[handler(ListTagsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "color": "#1F5AF6",
    //       "contact_count": 12,
    //       "created_at": "2026-04-19T16:00:00Z",
    //       "id": "8a4e41d9-bc47-5357-abf0-fe040418ee6e",
    //       "name": "customer"
    //     },
    //     {
    //       "color": "#0B7A4B",
    //       "contact_count": 6,
    //       "created_at": "2026-04-19T16:00:00Z",
    //       "id": "1d7a712a-45bb-51db-8b14-23fb2e8296ff",
    //       "name": "trial"
    //     },
    //     {
    //       "color": "#B45309",
    //       "contact_count": 3,
    //       "created_at": "2026-04-19T16:00:00Z",
    //       "id": "c0e89ee1-beaa-54aa-8af4-f16c1bdb152b",
    //       "name": "waitlist"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 12
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "color": "#1F5AF6",
      "contact_count": 12,
      "created_at": "2026-04-19T16:00:00Z",
      "id": "8a4e41d9-bc47-5357-abf0-fe040418ee6e",
      "name": "customer"
    },
    {
      "color": "#0B7A4B",
      "contact_count": 6,
      "created_at": "2026-04-19T16:00:00Z",
      "id": "1d7a712a-45bb-51db-8b14-23fb2e8296ff",
      "name": "trial"
    },
    {
      "color": "#B45309",
      "contact_count": 3,
      "created_at": "2026-04-19T16:00:00Z",
      "id": "c0e89ee1-beaa-54aa-8af4-f16c1bdb152b",
      "name": "waitlist"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 12
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Tag>(serde_json::json!({"color":"#1F5AF6","contact_count":12,"created_at":"2026-04-19T16:00:00Z","id":"8a4e41d9-bc47-5357-abf0-fe040418ee6e","name":"customer"})).unwrap_or_default(), serde_json::from_value::<Tag>(serde_json::json!({"color":"#0B7A4B","contact_count":6,"created_at":"2026-04-19T16:00:00Z","id":"1d7a712a-45bb-51db-8b14-23fb2e8296ff","name":"trial"})).unwrap_or_default(), serde_json::from_value::<Tag>(serde_json::json!({"color":"#B45309","contact_count":3,"created_at":"2026-04-19T16:00:00Z","id":"c0e89ee1-beaa-54aa-8af4-f16c1bdb152b","name":"waitlist"})).unwrap_or_default()],limit: 25,page: 1,total: 12,
    })
}
