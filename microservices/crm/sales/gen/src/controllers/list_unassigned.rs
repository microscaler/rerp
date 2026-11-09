// User-owned controller for handler 'list_unassigned'.

use crate::handlers::list_unassigned::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::UnassignedLead;

#[handler(ListUnassignedController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "created_at": "2026-11-05T16:00:00Z",
    //       "email_from": "Email from",
    //       "lead_id": "36ad9027-d2b6-5440-962c-1bf60325b115",
    //       "lead_name": "Lead name",
    //       "source": "Source"
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
      "created_at": "2026-11-05T16:00:00Z",
      "email_from": "Email from",
      "lead_id": "36ad9027-d2b6-5440-962c-1bf60325b115",
      "lead_name": "Lead name",
      "source": "Source"
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
        items: vec![serde_json::from_value::<UnassignedLead>(serde_json::json!({"created_at":"2026-11-05T16:00:00Z","email_from":"Email from","lead_id":"36ad9027-d2b6-5440-962c-1bf60325b115","lead_name":"Lead name","source":"Source"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
