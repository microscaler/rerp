// User-owned controller for handler 'add_contact_note'.

use crate::handlers::add_contact_note::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ActivityKind;

#[handler(AddContactNoteController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        actor: Some("example".to_string()),
        at: "example".to_string(),
        contact_id: "example".to_string(),
        data: Some(Default::default()),
        id: "example".to_string(),
        kind: Default::default(),
        ref_id: Some("example".to_string()),
        ref_service: Some("example".to_string()),
        summary: "example".to_string(),
    })
}
