// User-owned controller for handler 'list_team'.

use crate::handlers::list_team::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::TeamMember;

#[handler(ListTeamController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "can_approve_social": true,
    //       "can_send": true,
    //       "last_active_at": "2026-11-05T16:00:00Z",
    //       "name": "Alex Morgan",
    //       "role": "owner",
    //       "user_id": "alex"
    //     },
    //     {
    //       "can_approve_social": false,
    //       "can_send": false,
    //       "last_active_at": "2026-11-05T14:00:00Z",
    //       "name": "Sam Lee",
    //       "role": "owner",
    //       "user_id": "sam"
    //     },
    //     {
    //       "can_approve_social": true,
    //       "can_send": false,
    //       "last_active_at": "2026-11-04T16:00:00Z",
    //       "name": "Riya Kapoor",
    //       "role": "owner",
    //       "user_id": "riya"
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
      "can_approve_social": true,
      "can_send": true,
      "last_active_at": "2026-11-05T16:00:00Z",
      "name": "Alex Morgan",
      "role": "owner",
      "user_id": "alex"
    },
    {
      "can_approve_social": false,
      "can_send": false,
      "last_active_at": "2026-11-05T14:00:00Z",
      "name": "Sam Lee",
      "role": "owner",
      "user_id": "sam"
    },
    {
      "can_approve_social": true,
      "can_send": false,
      "last_active_at": "2026-11-04T16:00:00Z",
      "name": "Riya Kapoor",
      "role": "owner",
      "user_id": "riya"
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
        items: vec![serde_json::from_value::<TeamMember>(serde_json::json!({"can_approve_social":true,"can_send":true,"last_active_at":"2026-11-05T16:00:00Z","name":"Alex Morgan","role":"owner","user_id":"alex"})).unwrap_or_default(), serde_json::from_value::<TeamMember>(serde_json::json!({"can_approve_social":false,"can_send":false,"last_active_at":"2026-11-05T14:00:00Z","name":"Sam Lee","role":"owner","user_id":"sam"})).unwrap_or_default(), serde_json::from_value::<TeamMember>(serde_json::json!({"can_approve_social":true,"can_send":false,"last_active_at":"2026-11-04T16:00:00Z","name":"Riya Kapoor","role":"owner","user_id":"riya"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
