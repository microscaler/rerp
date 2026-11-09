// User-owned controller for handler 'verify_tracking_domain'.

use crate::handlers::verify_tracking_domain::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(VerifyTrackingDomainController)]
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
