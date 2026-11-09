// User-owned controller for handler 'list_form_submissions'.

use crate::handlers::list_form_submissions::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::FormSubmission;

#[handler(ListFormSubmissionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "form_id": "c49b4999-d0ed-58c1-a824-1ffe42b213e8",
    //       "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
    //       "submitted_at": "2026-11-05T16:00:00Z",
    //       "values": {
    //         "key": "Value"
    //       }
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "form_id": "c49b4999-d0ed-58c1-a824-1ffe42b213e8",
      "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
      "submitted_at": "2026-11-05T16:00:00Z",
      "values": {
        "key": "Value"
      }
    }
  ],
  "limit": 25,
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
        items: vec![serde_json::from_value::<FormSubmission>(serde_json::json!({"form_id":"c49b4999-d0ed-58c1-a824-1ffe42b213e8","id":"e67555dd-33f4-5fee-9da3-a019895219b1","submitted_at":"2026-11-05T16:00:00Z","values":{"key":"Value"}})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
