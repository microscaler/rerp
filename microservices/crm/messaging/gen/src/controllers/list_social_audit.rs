// User-owned controller for handler 'list_social_audit'.

use crate::handlers::list_social_audit::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AuditEntry;

#[handler(ListSocialAuditController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "action": "CREATED",
    //       "actor": "Actor",
    //       "at": "2026-11-05T16:00:00Z",
    //       "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76"
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
      "action": "CREATED",
      "actor": "Actor",
      "at": "2026-11-05T16:00:00Z",
      "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76"
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
        items: vec![serde_json::from_value::<AuditEntry>(serde_json::json!({"action":"CREATED","actor":"Actor","at":"2026-11-05T16:00:00Z","id":"3eb009b7-a99d-5a4c-956a-99e11d8dbb76"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
