// User-owned controller for handler 'update_postal_address'.

use crate::handlers::update_postal_address::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdatePostalAddressController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        address: "example".to_string(),
        id: "example".to_string(),
        is_default: Some(true),
        name: "example".to_string(),
        used_by: Some(vec![]),
    })
}
