// User-owned controller for handler 'add_step_note'.

use crate::handlers::add_step_note::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(AddStepNoteController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        author: Some("example".to_string()),
        created_at: Some("example".to_string()),
        id: Some("example".to_string()),
        text: "example".to_string(),
    })
}
