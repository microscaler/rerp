// User-owned controller for handler 'record_ad_click'.

use crate::handlers::record_ad_click::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RecordAdClickController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "click_id": "c959d5fb-5951-55ce-88a2-1a2a0c57359b",
    //   "consent_applied": "FULL",
    //   "contact_id": "1f8a19e4-e646-5150-9e90-d62be2bd2fcf",
    //   "set_cookie": false,
    //   "visitor_id": "v_new"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "click_id": "c959d5fb-5951-55ce-88a2-1a2a0c57359b",
  "consent_applied": "FULL",
  "contact_id": "1f8a19e4-e646-5150-9e90-d62be2bd2fcf",
  "set_cookie": false,
  "visitor_id": "v_new"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        click_id: Some("c959d5fb-5951-55ce-88a2-1a2a0c57359b".to_string()),
        consent_applied: Some("FULL".to_string()),
        contact_id: Some("1f8a19e4-e646-5150-9e90-d62be2bd2fcf".to_string()),
        set_cookie: Some(false),
        visitor_id: Some("v_new".to_string()),
    })
}
