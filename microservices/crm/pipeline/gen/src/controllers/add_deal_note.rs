// User-owned controller for handler 'add_deal_note'.

use crate::handlers::add_deal_note::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(AddDealNoteController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        author: Some("example".to_string()),
        created_at: "example".to_string(),
        id: "example".to_string(),
        pinned: Some(true),
        text: "example".to_string(),
        updated_at: Some("example".to_string()),
    })
}
