// User-owned controller for handler 'update_snippet'.

use crate::handlers::update_snippet::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateSnippetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
    //   "key": "snippet.signature",
    //   "updated_at": "2026-11-05T16:00:00Z",
    //   "used_by": 3,
    //   "value": "Value"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "created_at": "2026-11-05T16:00:00Z",
  "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
  "key": "snippet.signature",
  "updated_at": "2026-11-05T16:00:00Z",
  "used_by": 3,
  "value": "Value"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        created_at: Some("2026-11-05T16:00:00Z".to_string()),
        id: "3eb009b7-a99d-5a4c-956a-99e11d8dbb76".to_string(),
        key: "snippet.signature".to_string(),
        updated_at: Some("2026-11-05T16:00:00Z".to_string()),
        used_by: Some(3),
        value: "Value".to_string(),
    })
}
