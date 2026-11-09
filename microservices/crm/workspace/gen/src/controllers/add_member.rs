// User-owned controller for handler 'add_member'.

use crate::handlers::add_member::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::WorkspaceRole;

#[handler(AddMemberController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        added_at: Some("example".to_string()),
        email: Some("example".to_string()),
        last_active_at: Some("example".to_string()),
        name: Some("example".to_string()),
        role: Default::default(),
        user_id: "example".to_string(),
    })
}
