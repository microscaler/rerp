// User-owned controller for handler 'get_ad_click'.

use crate::handlers::get_ad_click::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetAdClickController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        account_id: Some("example".to_string()),
        ad: Some("example".to_string()),
        ad_set: Some("example".to_string()),
        at: "example".to_string(),
        campaign: Some("example".to_string()),
        click_id_param: Some("example".to_string()),
        click_id_value: Some("example".to_string()),
        consent_applied: Some("example".to_string()),
        contact_id: Some("example".to_string()),
        contact_name: Some("example".to_string()),
        id: "example".to_string(),
        landing_url: Some("example".to_string()),
        platform: Some("example".to_string()),
        referrer: Some("example".to_string()),
        touchpoints: Some(42),
        utm: Some(Default::default()),
        visitor_id: Some("example".to_string()),
    })
}
