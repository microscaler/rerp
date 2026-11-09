// User-owned controller for handler 'audience_update_tag'.

use crate::handlers::audience_update_tag::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(AudienceUpdateTagController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "color": "#1F5AF6",
    //   "contact_count": 12,
    //   "created_at": "2026-04-19T16:00:00Z",
    //   "id": "8a4e41d9-bc47-5357-abf0-fe040418ee6e",
    //   "name": "customer"
    // }
    match serde_json::from_str::<Response>(
        r##"{
  "color": "#1F5AF6",
  "contact_count": 12,
  "created_at": "2026-04-19T16:00:00Z",
  "id": "8a4e41d9-bc47-5357-abf0-fe040418ee6e",
  "name": "customer"
}"##,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        color: Some("#1F5AF6".to_string()),
        contact_count: Some(12),
        created_at: Some("2026-04-19T16:00:00Z".to_string()),
        description: Some("example".to_string()),
        id: "8a4e41d9-bc47-5357-abf0-fe040418ee6e".to_string(),
        name: "customer".to_string(),
    })
}
