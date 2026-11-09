// User-owned controller for handler 'get_brand_kit'.

use crate::handlers::get_brand_kit::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetBrandKitController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        body_font: Some("example".to_string()),
        button_style: Some("example".to_string()),
        colors: Some(Default::default()),
        heading_font: Some("example".to_string()),
        id: "example".to_string(),
        logo_url: Some("example".to_string()),
        social_links: Some(vec![]),
    })
}
