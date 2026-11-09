// User-owned controller for handler 'list_api_keys'.

use crate::handlers::list_api_keys::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ApiKey;

#[handler(ListApiKeysController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "created_at": "2026-07-28T16:00:00Z",
    //       "id": "b90438d1-e134-5543-919f-19864d0919d3",
    //       "is_active": true,
    //       "name": "Website (public tracking key)",
    //       "scopes": {}
    //     },
    //     {
    //       "created_at": "2026-07-28T16:00:00Z",
    //       "id": "08dd68b1-5f07-5687-b207-d5b9cf18daaf",
    //       "is_active": true,
    //       "name": "Product backend",
    //       "scopes": {}
    //     },
    //     {
    //       "created_at": "2026-07-28T16:00:00Z",
    //       "id": "02a937b8-4dcd-59b0-9797-dcefad332287",
    //       "is_active": true,
    //       "name": "BI export",
    //       "scopes": {}
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 3
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "created_at": "2026-07-28T16:00:00Z",
      "id": "b90438d1-e134-5543-919f-19864d0919d3",
      "is_active": true,
      "name": "Website (public tracking key)",
      "scopes": {}
    },
    {
      "created_at": "2026-07-28T16:00:00Z",
      "id": "08dd68b1-5f07-5687-b207-d5b9cf18daaf",
      "is_active": true,
      "name": "Product backend",
      "scopes": {}
    },
    {
      "created_at": "2026-07-28T16:00:00Z",
      "id": "02a937b8-4dcd-59b0-9797-dcefad332287",
      "is_active": true,
      "name": "BI export",
      "scopes": {}
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 3
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<ApiKey>(serde_json::json!({"created_at":"2026-07-28T16:00:00Z","id":"b90438d1-e134-5543-919f-19864d0919d3","is_active":true,"name":"Website (public tracking key)","scopes":{}})).unwrap_or_default(), serde_json::from_value::<ApiKey>(serde_json::json!({"created_at":"2026-07-28T16:00:00Z","id":"08dd68b1-5f07-5687-b207-d5b9cf18daaf","is_active":true,"name":"Product backend","scopes":{}})).unwrap_or_default(), serde_json::from_value::<ApiKey>(serde_json::json!({"created_at":"2026-07-28T16:00:00Z","id":"02a937b8-4dcd-59b0-9797-dcefad332287","is_active":true,"name":"BI export","scopes":{}})).unwrap_or_default()],limit: 25,page: 1,total: 3,
    })
}
