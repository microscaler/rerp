// User-owned controller for handler 'create_suppression'.

use crate::handlers::create_suppression::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Channel;
#[allow(unused_imports)]
use crate::handlers::types::SuppressionReason;

#[handler(CreateSuppressionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        address: "example".to_string(),
        channel: Some(Default::default()),
        created_at: "example".to_string(),
        id: "example".to_string(),
        note: Some("example".to_string()),
        reason: Default::default(),
        source_send_id: Some("example".to_string()),
    })
}
