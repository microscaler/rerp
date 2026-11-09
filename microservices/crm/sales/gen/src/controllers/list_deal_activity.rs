// User-owned controller for handler 'list_deal_activity'.

use crate::handlers::list_deal_activity::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ActivityEntry;

#[handler(ListDealActivityController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "actor": "mailbox",
    //       "at": "2026-11-05T11:00:00Z",
    //       "deal_id": "63a6661f-5860-50ed-8697-f28d29a3917f",
    //       "id": "3e3541c5-986e-5136-bf06-7fdcf58b3b41",
    //       "kind": "EMAIL_RECEIVED",
    //       "summary": "Email received from Priya Natarajan"
    //     },
    //     {
    //       "actor": "Alex Morgan",
    //       "at": "2026-11-04T14:00:00Z",
    //       "deal_id": "63a6661f-5860-50ed-8697-f28d29a3917f",
    //       "id": "56f9f35a-b3ae-561e-a01d-fc17a886c66f",
    //       "kind": "CREATED",
    //       "summary": "Note added"
    //     },
    //     {
    //       "actor": "Alex Morgan",
    //       "at": "2026-11-04T11:00:00Z",
    //       "deal_id": "63a6661f-5860-50ed-8697-f28d29a3917f",
    //       "id": "49dfa063-a2e2-5b6b-8052-24d0928bc8d4",
    //       "kind": "EMAIL_SENT",
    //       "summary": "1:1 email sent · Proposal for 120 seats"
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 6
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "actor": "mailbox",
      "at": "2026-11-05T11:00:00Z",
      "deal_id": "63a6661f-5860-50ed-8697-f28d29a3917f",
      "id": "3e3541c5-986e-5136-bf06-7fdcf58b3b41",
      "kind": "EMAIL_RECEIVED",
      "summary": "Email received from Priya Natarajan"
    },
    {
      "actor": "Alex Morgan",
      "at": "2026-11-04T14:00:00Z",
      "deal_id": "63a6661f-5860-50ed-8697-f28d29a3917f",
      "id": "56f9f35a-b3ae-561e-a01d-fc17a886c66f",
      "kind": "CREATED",
      "summary": "Note added"
    },
    {
      "actor": "Alex Morgan",
      "at": "2026-11-04T11:00:00Z",
      "deal_id": "63a6661f-5860-50ed-8697-f28d29a3917f",
      "id": "49dfa063-a2e2-5b6b-8052-24d0928bc8d4",
      "kind": "EMAIL_SENT",
      "summary": "1:1 email sent · Proposal for 120 seats"
    }
  ],
  "limit": 20,
  "page": 1,
  "total": 6
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<ActivityEntry>(serde_json::json!({"actor":"mailbox","at":"2026-11-05T11:00:00Z","deal_id":"63a6661f-5860-50ed-8697-f28d29a3917f","id":"3e3541c5-986e-5136-bf06-7fdcf58b3b41","kind":"EMAIL_RECEIVED","summary":"Email received from Priya Natarajan"})).unwrap_or_default(), serde_json::from_value::<ActivityEntry>(serde_json::json!({"actor":"Alex Morgan","at":"2026-11-04T14:00:00Z","deal_id":"63a6661f-5860-50ed-8697-f28d29a3917f","id":"56f9f35a-b3ae-561e-a01d-fc17a886c66f","kind":"CREATED","summary":"Note added"})).unwrap_or_default(), serde_json::from_value::<ActivityEntry>(serde_json::json!({"actor":"Alex Morgan","at":"2026-11-04T11:00:00Z","deal_id":"63a6661f-5860-50ed-8697-f28d29a3917f","id":"49dfa063-a2e2-5b6b-8052-24d0928bc8d4","kind":"EMAIL_SENT","summary":"1:1 email sent · Proposal for 120 seats"})).unwrap_or_default()],limit: 20,page: 1,total: 6,
    })
}
