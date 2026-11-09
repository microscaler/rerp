// User-owned controller for handler 'get_compliance'.

use crate::handlers::get_compliance::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetComplianceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "anonymous_retention_days": 90,
    //   "double_opt_in_default": true,
    //   "footer_postal_address": "Acme Co, 12 Example Street, 10115 Berlin, Germany",
    //   "one_click_unsubscribe": true,
    //   "require_unsubscribe_links": true,
    //   "updated_at": "2026-10-24T16:00:00Z"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "anonymous_retention_days": 90,
  "double_opt_in_default": true,
  "footer_postal_address": "Acme Co, 12 Example Street, 10115 Berlin, Germany",
  "one_click_unsubscribe": true,
  "require_unsubscribe_links": true,
  "updated_at": "2026-10-24T16:00:00Z"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        anonymous_retention_days: Some(90),
        cookie_consent_mode: Some("example".to_string()),
        double_opt_in_default: Some(true),
        double_opt_in_regions: Some(vec![]),
        footer_postal_address: Some(
            "Acme Co, 12 Example Street, 10115 Berlin, Germany".to_string(),
        ),
        legitimate_interest_allowed: Some(true),
        one_click_unsubscribe: Some(true),
        pending_signup_expiry_days: Some(42),
        require_unsubscribe_links: Some(true),
        updated_at: Some("2026-10-24T16:00:00Z".to_string()),
    })
}
