// User-owned controller for handler 'create_tracked_site'.

use crate::handlers::create_tracked_site::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateTrackedSiteController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "created_at": "2026-05-09T16:00:00Z",
    //   "id": "f3a558f4-397d-5377-8279-803f527fe211",
    //   "last_event_at": "2026-11-05T15:59:48Z",
    //   "name": "Marketing site",
    //   "origin": "https://acme.example",
    //   "tracking_key": "pk_site_07c1e"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "created_at": "2026-05-09T16:00:00Z",
  "id": "f3a558f4-397d-5377-8279-803f527fe211",
  "last_event_at": "2026-11-05T15:59:48Z",
  "name": "Marketing site",
  "origin": "https://acme.example",
  "tracking_key": "pk_site_07c1e"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        created_at: "2026-05-09T16:00:00Z".to_string(),
        enabled: Some(true),
        id: "f3a558f4-397d-5377-8279-803f527fe211".to_string(),
        include_all_pages: Some(true),
        last_event_at: Some("2026-11-05T15:59:48Z".to_string()),
        name: Some("Marketing site".to_string()),
        origin: "https://acme.example".to_string(),
        snippet: Some("example".to_string()),
        tracking_key: "pk_site_07c1e".to_string(),
    })
}
