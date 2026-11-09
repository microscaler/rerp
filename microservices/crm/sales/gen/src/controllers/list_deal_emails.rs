// User-owned controller for handler 'list_deal_emails'.

use crate::handlers::list_deal_emails::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DealEmail;

#[handler(ListDealEmailsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "at": "2026-11-05T16:00:00Z",
    //       "clicks": 0,
    //       "deal_id": "63a6661f-5860-50ed-8697-f28d29a3917f",
    //       "direction": "OUT",
    //       "from": "Priya Natarajan",
    //       "id": "b677a779-a5b0-5d78-b4a8-98779d4bdb4b",
    //       "kind": "MAILBOX",
    //       "opens": 0,
    //       "sent_at": "2026-11-05T11:00:00Z",
    //       "subject": "Re: Proposal for 120 seats",
    //       "to": [
    //         "To"
    //       ]
    //     },
    //     {
    //       "at": "2026-11-05T16:00:00Z",
    //       "clicks": 1,
    //       "deal_id": "63a6661f-5860-50ed-8697-f28d29a3917f",
    //       "direction": "OUT",
    //       "from": "Alex Morgan",
    //       "id": "deec010c-855c-539b-8d81-1431924ccb88",
    //       "kind": "ONE_TO_ONE",
    //       "opens": 2,
    //       "sent_at": "2026-11-05T05:00:00Z",
    //       "subject": "Proposal for 120 seats",
    //       "to": [
    //         "To"
    //       ]
    //     },
    //     {
    //       "at": "2026-11-05T16:00:00Z",
    //       "clicks": 0,
    //       "deal_id": "63a6661f-5860-50ed-8697-f28d29a3917f",
    //       "direction": "OUT",
    //       "from": "Campaign",
    //       "id": "1c2c13d1-7fe2-5067-bfa7-0039c31d4603",
    //       "kind": "CAMPAIGN",
    //       "opens": 1,
    //       "sent_at": "2026-10-28T16:00:00Z",
    //       "subject": "October product update",
    //       "to": [
    //         "To"
    //       ]
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 3
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "at": "2026-11-05T16:00:00Z",
      "clicks": 0,
      "deal_id": "63a6661f-5860-50ed-8697-f28d29a3917f",
      "direction": "OUT",
      "from": "Priya Natarajan",
      "id": "b677a779-a5b0-5d78-b4a8-98779d4bdb4b",
      "kind": "MAILBOX",
      "opens": 0,
      "sent_at": "2026-11-05T11:00:00Z",
      "subject": "Re: Proposal for 120 seats",
      "to": [
        "To"
      ]
    },
    {
      "at": "2026-11-05T16:00:00Z",
      "clicks": 1,
      "deal_id": "63a6661f-5860-50ed-8697-f28d29a3917f",
      "direction": "OUT",
      "from": "Alex Morgan",
      "id": "deec010c-855c-539b-8d81-1431924ccb88",
      "kind": "ONE_TO_ONE",
      "opens": 2,
      "sent_at": "2026-11-05T05:00:00Z",
      "subject": "Proposal for 120 seats",
      "to": [
        "To"
      ]
    },
    {
      "at": "2026-11-05T16:00:00Z",
      "clicks": 0,
      "deal_id": "63a6661f-5860-50ed-8697-f28d29a3917f",
      "direction": "OUT",
      "from": "Campaign",
      "id": "1c2c13d1-7fe2-5067-bfa7-0039c31d4603",
      "kind": "CAMPAIGN",
      "opens": 1,
      "sent_at": "2026-10-28T16:00:00Z",
      "subject": "October product update",
      "to": [
        "To"
      ]
    }
  ],
  "limit": 20,
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
        items: vec![serde_json::from_value::<DealEmail>(serde_json::json!({"at":"2026-11-05T16:00:00Z","clicks":0,"deal_id":"63a6661f-5860-50ed-8697-f28d29a3917f","direction":"OUT","from":"Priya Natarajan","id":"b677a779-a5b0-5d78-b4a8-98779d4bdb4b","kind":"MAILBOX","opens":0,"sent_at":"2026-11-05T11:00:00Z","subject":"Re: Proposal for 120 seats","to":["To"]})).unwrap_or_default(), serde_json::from_value::<DealEmail>(serde_json::json!({"at":"2026-11-05T16:00:00Z","clicks":1,"deal_id":"63a6661f-5860-50ed-8697-f28d29a3917f","direction":"OUT","from":"Alex Morgan","id":"deec010c-855c-539b-8d81-1431924ccb88","kind":"ONE_TO_ONE","opens":2,"sent_at":"2026-11-05T05:00:00Z","subject":"Proposal for 120 seats","to":["To"]})).unwrap_or_default(), serde_json::from_value::<DealEmail>(serde_json::json!({"at":"2026-11-05T16:00:00Z","clicks":0,"deal_id":"63a6661f-5860-50ed-8697-f28d29a3917f","direction":"OUT","from":"Campaign","id":"1c2c13d1-7fe2-5067-bfa7-0039c31d4603","kind":"CAMPAIGN","opens":1,"sent_at":"2026-10-28T16:00:00Z","subject":"October product update","to":["To"]})).unwrap_or_default()],limit: 20,page: 1,total: 3,
    })
}
