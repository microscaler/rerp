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
    //       "direction": "OUT",
    //       "from": "Priya Natarajan",
    //       "id": "b677a779-a5b0-5d78-b4a8-98779d4bdb4b",
    //       "subject": "Re: Proposal for 120 seats",
    //       "to": [
    //         "To"
    //       ]
    //     },
    //     {
    //       "at": "2026-11-05T16:00:00Z",
    //       "direction": "OUT",
    //       "from": "Alex Morgan",
    //       "id": "deec010c-855c-539b-8d81-1431924ccb88",
    //       "subject": "Proposal for 120 seats",
    //       "to": [
    //         "To"
    //       ]
    //     },
    //     {
    //       "at": "2026-11-05T16:00:00Z",
    //       "direction": "OUT",
    //       "from": "Campaign",
    //       "id": "1c2c13d1-7fe2-5067-bfa7-0039c31d4603",
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
        r###"{
  "items": [
    {
      "at": "2026-11-05T16:00:00Z",
      "direction": "OUT",
      "from": "Priya Natarajan",
      "id": "b677a779-a5b0-5d78-b4a8-98779d4bdb4b",
      "subject": "Re: Proposal for 120 seats",
      "to": [
        "To"
      ]
    },
    {
      "at": "2026-11-05T16:00:00Z",
      "direction": "OUT",
      "from": "Alex Morgan",
      "id": "deec010c-855c-539b-8d81-1431924ccb88",
      "subject": "Proposal for 120 seats",
      "to": [
        "To"
      ]
    },
    {
      "at": "2026-11-05T16:00:00Z",
      "direction": "OUT",
      "from": "Campaign",
      "id": "1c2c13d1-7fe2-5067-bfa7-0039c31d4603",
      "subject": "October product update",
      "to": [
        "To"
      ]
    }
  ],
  "limit": 20,
  "page": 1,
  "total": 3
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<DealEmail>(serde_json::json!({"at":"2026-11-05T16:00:00Z","direction":"OUT","from":"Priya Natarajan","id":"b677a779-a5b0-5d78-b4a8-98779d4bdb4b","subject":"Re: Proposal for 120 seats","to":["To"]})).unwrap_or_default(), serde_json::from_value::<DealEmail>(serde_json::json!({"at":"2026-11-05T16:00:00Z","direction":"OUT","from":"Alex Morgan","id":"deec010c-855c-539b-8d81-1431924ccb88","subject":"Proposal for 120 seats","to":["To"]})).unwrap_or_default(), serde_json::from_value::<DealEmail>(serde_json::json!({"at":"2026-11-05T16:00:00Z","direction":"OUT","from":"Campaign","id":"1c2c13d1-7fe2-5067-bfa7-0039c31d4603","subject":"October product update","to":["To"]})).unwrap_or_default()],limit: 20,page: 1,total: 3,
    })
}
