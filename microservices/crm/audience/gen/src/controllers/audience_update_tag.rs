// User-owned controller for handler 'audience_update_tag'.

use crate::handlers::audience_update_tag::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(AudienceUpdateTagController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        color: Some("example".to_string()),
        contact_count: Some(42),
        created_at: Some("example".to_string()),
        description: Some("example".to_string()),
        id: "example".to_string(),
        name: "example".to_string(),
    })
}
