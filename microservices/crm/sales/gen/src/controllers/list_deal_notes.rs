// User-owned controller for handler 'list_deal_notes'.

use crate::handlers::list_deal_notes::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Note;

#[handler(ListDealNotesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "author": "Alex Morgan",
    //       "created_at": "2026-11-04T14:00:00Z",
    //       "id": "c5a53e54-861c-5955-9c05-355ff0f0ab0c",
    //       "text": "Priya confirmed budget approved for 120 seats; SSO is the blocker. Sent SAML docs."
    //     },
    //     {
    //       "author": "Automation · Hot lead handoff",
    //       "created_at": "2026-11-02T09:00:00Z",
    //       "id": "938887ff-e5fb-5b68-b294-071e744f7951",
    //       "text": "Deal created from form \"Contact sales\" (seats = 120)."
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "author": "Alex Morgan",
      "created_at": "2026-11-04T14:00:00Z",
      "id": "c5a53e54-861c-5955-9c05-355ff0f0ab0c",
      "text": "Priya confirmed budget approved for 120 seats; SSO is the blocker. Sent SAML docs."
    },
    {
      "author": "Automation · Hot lead handoff",
      "created_at": "2026-11-02T09:00:00Z",
      "id": "938887ff-e5fb-5b68-b294-071e744f7951",
      "text": "Deal created from form \"Contact sales\" (seats = 120)."
    }
  ],
  "limit": 20,
  "page": 1,
  "total": 2
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Note>(serde_json::json!({"author":"Alex Morgan","created_at":"2026-11-04T14:00:00Z","id":"c5a53e54-861c-5955-9c05-355ff0f0ab0c","text":"Priya confirmed budget approved for 120 seats; SSO is the blocker. Sent SAML docs."})).unwrap_or_default(), serde_json::from_value::<Note>(serde_json::json!({"author":"Automation · Hot lead handoff","created_at":"2026-11-02T09:00:00Z","id":"938887ff-e5fb-5b68-b294-071e744f7951","text":"Deal created from form \"Contact sales\" (seats = 120)."})).unwrap_or_default()],limit: 20,page: 1,total: 2,
    })
}
