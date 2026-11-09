// User-owned controller for handler 'update_deal_note'.

use crate::handlers::update_deal_note::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateDealNoteController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "author": "Author",
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "deal_id": "22fefcba-8554-529b-a78d-271dec0488c7",
    //   "id": "1b23e5c3-0ff4-5d6d-a54b-e72da751ce26",
    //   "pinned": false,
    //   "text": "Thanks for getting in touch.",
    //   "updated_at": "2026-11-05T16:00:00Z"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "author": "Author",
  "created_at": "2026-11-05T16:00:00Z",
  "deal_id": "22fefcba-8554-529b-a78d-271dec0488c7",
  "id": "1b23e5c3-0ff4-5d6d-a54b-e72da751ce26",
  "pinned": false,
  "text": "Thanks for getting in touch.",
  "updated_at": "2026-11-05T16:00:00Z"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        author: Some("Author".to_string()),
        created_at: "2026-11-05T16:00:00Z".to_string(),
        deal_id: Some("22fefcba-8554-529b-a78d-271dec0488c7".to_string()),
        id: "1b23e5c3-0ff4-5d6d-a54b-e72da751ce26".to_string(),
        pinned: Some(false),
        text: "Thanks for getting in touch.".to_string(),
        updated_at: Some("2026-11-05T16:00:00Z".to_string()),
    })
}
