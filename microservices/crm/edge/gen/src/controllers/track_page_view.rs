// User-owned controller for handler 'track_page_view'.

use crate::handlers::track_page_view::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(TrackPageViewController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "city": "City",
    //   "converted_to_lead": false,
    //   "country": "DE",
    //   "first_seen": "2026-11-05T16:00:00Z",
    //   "id": "9fa51332-6221-502c-a499-80ae6ddde550",
    //   "last_seen": "2026-11-05T16:00:00Z",
    //   "lead_id": "b68dbf73-2479-5234-b517-798dd1265706",
    //   "pages_viewed": 3,
    //   "referrer": "Referrer",
    //   "url_visited": "Url visited",
    //   "user_agent": "User agent",
    //   "visitor_id": "ext_20eadbef80"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "city": "City",
  "converted_to_lead": false,
  "country": "DE",
  "first_seen": "2026-11-05T16:00:00Z",
  "id": "9fa51332-6221-502c-a499-80ae6ddde550",
  "last_seen": "2026-11-05T16:00:00Z",
  "lead_id": "b68dbf73-2479-5234-b517-798dd1265706",
  "pages_viewed": 3,
  "referrer": "Referrer",
  "url_visited": "Url visited",
  "user_agent": "User agent",
  "visitor_id": "ext_20eadbef80"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        city: Some("City".to_string()),
        converted_to_lead: Some(false),
        country: Some("DE".to_string()),
        first_seen: "2026-11-05T16:00:00Z".to_string(),
        id: "9fa51332-6221-502c-a499-80ae6ddde550".to_string(),
        last_seen: Some("2026-11-05T16:00:00Z".to_string()),
        lead_id: Some("b68dbf73-2479-5234-b517-798dd1265706".to_string()),
        pages_viewed: Some(3),
        referrer: Some("Referrer".to_string()),
        url_visited: Some("Url visited".to_string()),
        user_agent: Some("User agent".to_string()),
        visitor_id: "ext_20eadbef80".to_string(),
    })
}
