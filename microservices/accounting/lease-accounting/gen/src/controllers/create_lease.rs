// User-owned controller for handler 'create_lease'.

use crate::handlers::create_lease::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateLeaseController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        classification: Default::default(),
        commencement_date: "example".to_string(),
        discount_rate: Some(3.14),
        id: "example".to_string(),
        lease_number: "example".to_string(),
        status: Default::default(),
        termination_date: Some("example".to_string()),
    }
}
