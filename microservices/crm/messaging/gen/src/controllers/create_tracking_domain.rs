// User-owned controller for handler 'create_tracking_domain'.

use crate::handlers::create_tracking_domain::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateTrackingDomainController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        domain: "example".to_string(),
        id: "example".to_string(),
        last_checked_at: Some("example".to_string()),
        records: Some(vec![]),
        status: "example".to_string(),
        tls: Some(true),
    })
}
