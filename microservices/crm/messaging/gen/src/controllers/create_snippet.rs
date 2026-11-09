// User-owned controller for handler 'create_snippet'.

use crate::handlers::create_snippet::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateSnippetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "created_at": "2026-07-28T16:00:00Z",
    //   "id": "d2784443-bc70-5675-8a68-30620c50a0eb",
    //   "key": "snippet.signature",
    //   "used_by": 6,
    //   "value": "Alex Morgan · Head of Marketing · acme.example"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "created_at": "2026-07-28T16:00:00Z",
  "id": "d2784443-bc70-5675-8a68-30620c50a0eb",
  "key": "snippet.signature",
  "used_by": 6,
  "value": "Alex Morgan · Head of Marketing · acme.example"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        created_at: Some("2026-07-28T16:00:00Z".to_string()),
        id: "d2784443-bc70-5675-8a68-30620c50a0eb".to_string(),
        key: "snippet.signature".to_string(),
        updated_at: Some("example".to_string()),
        used_by: Some(6),
        value: "Alex Morgan · Head of Marketing · acme.example".to_string(),
    })
}
