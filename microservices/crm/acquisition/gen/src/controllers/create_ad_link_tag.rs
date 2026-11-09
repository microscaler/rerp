// User-owned controller for handler 'create_ad_link_tag'.

use crate::handlers::create_ad_link_tag::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateAdLinkTagController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        add_click_id: Some(true),
        add_contact_token: Some(true),
        applies_to: "example".to_string(),
        domains: Some(vec![]),
        enabled: Some(true),
        id: Some("example".to_string()),
        utm_campaign: Some("example".to_string()),
        utm_content: Some("example".to_string()),
        utm_medium: Some("example".to_string()),
        utm_source: Some("example".to_string()),
    })
}
