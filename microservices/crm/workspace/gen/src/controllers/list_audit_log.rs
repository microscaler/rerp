// User-owned controller for handler 'list_audit_log'.

use crate::handlers::list_audit_log::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AuditLogEntry;

#[handler(ListAuditLogController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "action": "CREATE",
    //       "created_at": "2026-11-05T16:00:00Z",
    //       "entity": "Entity",
    //       "entity_id": "ext_58da2141b0",
    //       "id": "94b293ed-258b-5091-8e79-75d2ac471f25",
    //       "user_id": "alex"
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
      "action": "CREATE",
      "created_at": "2026-11-05T16:00:00Z",
      "entity": "Entity",
      "entity_id": "ext_58da2141b0",
      "id": "94b293ed-258b-5091-8e79-75d2ac471f25",
      "user_id": "alex"
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
        items: vec![serde_json::from_value::<AuditLogEntry>(serde_json::json!({"action":"CREATE","created_at":"2026-11-05T16:00:00Z","entity":"Entity","entity_id":"ext_58da2141b0","id":"94b293ed-258b-5091-8e79-75d2ac471f25","user_id":"alex"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
