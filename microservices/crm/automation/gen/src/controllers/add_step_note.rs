// User-owned controller for handler 'add_step_note'.

use crate::handlers::add_step_note::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(AddStepNoteController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "author": "Author",
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
    //   "text": "Thanks for getting in touch."
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "author": "Author",
  "created_at": "2026-11-05T16:00:00Z",
  "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
  "text": "Thanks for getting in touch."
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        author: Some("Author".to_string()),
        created_at: Some("2026-11-05T16:00:00Z".to_string()),
        id: Some("c9e18116-9fea-5e6e-ac32-52580d2450b8".to_string()),
        text: "Thanks for getting in touch.".to_string(),
    })
}
