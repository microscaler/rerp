// User-owned controller for handler 'add_deal_note'.

use crate::handlers::add_deal_note::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(AddDealNoteController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "author": "Me",
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "id": "2e459265-db4e-592e-aa0a-de7b7189dc8e",
    //   "text": ""
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "author": "Me",
  "created_at": "2026-11-05T16:00:00Z",
  "id": "2e459265-db4e-592e-aa0a-de7b7189dc8e",
  "text": ""
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        author: Some("Me".to_string()),
        created_at: "2026-11-05T16:00:00Z".to_string(),
        id: "2e459265-db4e-592e-aa0a-de7b7189dc8e".to_string(),
        pinned: Some(true),
        text: "".to_string(),
        updated_at: Some("example".to_string()),
    })
}
