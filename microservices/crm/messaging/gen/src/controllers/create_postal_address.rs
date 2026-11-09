// User-owned controller for handler 'create_postal_address'.

use crate::handlers::create_postal_address::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreatePostalAddressController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        address: "example".to_string(),
        id: "example".to_string(),
        is_default: Some(true),
        name: "example".to_string(),
        used_by: Some(vec![]),
    })
}
