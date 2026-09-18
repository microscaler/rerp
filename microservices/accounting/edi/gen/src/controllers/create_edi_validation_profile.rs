// User-owned controller for handler 'create_edi_validation_profile'.

use crate::handlers::create_edi_validation_profile::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateEdiValidationProfileController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        active: true,
        id: "example".to_string(),
        name: "example".to_string(),
        profile_id: "example".to_string(),
        rules: Some(Default::default()),
    })
}
