// User-owned controller for handler 'render_saved_response'.

use crate::handlers::render_saved_response::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RenderSavedResponseController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        body: Some("example".to_string()),
        subject: Some("example".to_string()),
    })
}
