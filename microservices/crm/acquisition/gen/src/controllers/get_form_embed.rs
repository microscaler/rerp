// User-owned controller for handler 'get_form_embed'.

use crate::handlers::get_form_embed::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetFormEmbedController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        form_id: "example".to_string(),
        hosted_url: "example".to_string(),
        inline_html: Some("example".to_string()),
        script_tag: "example".to_string(),
    })
}
