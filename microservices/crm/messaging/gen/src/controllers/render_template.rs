// User-owned controller for handler 'render_template'.

use crate::handlers::render_template::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RenderTemplateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        html: "example".to_string(),
        preheader: Some("example".to_string()),
        subject: Some("example".to_string()),
        text: "example".to_string(),
        unresolved: Some(vec![]),
    })
}
