// User-owned controller for handler 'record_ad_click'.

use crate::handlers::record_ad_click::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RecordAdClickController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        click_id: Some("example".to_string()),
        consent_applied: Some("example".to_string()),
        contact_id: Some("example".to_string()),
        set_cookie: Some(true),
        visitor_id: Some("example".to_string()),
    })
}
