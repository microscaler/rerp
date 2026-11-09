// User-owned controller for handler 'get_compliance'.

use crate::handlers::get_compliance::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetComplianceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        anonymous_retention_days: Some(42),
        cookie_consent_mode: Some("example".to_string()),
        double_opt_in_default: Some(true),
        double_opt_in_regions: Some(vec![]),
        footer_postal_address: Some("example".to_string()),
        legitimate_interest_allowed: Some(true),
        one_click_unsubscribe: Some(true),
        pending_signup_expiry_days: Some(42),
        require_unsubscribe_links: Some(true),
        updated_at: Some("example".to_string()),
    })
}
