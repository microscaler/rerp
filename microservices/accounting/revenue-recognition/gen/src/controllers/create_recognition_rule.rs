// User-owned controller for handler 'create_recognition_rule'.

use crate::handlers::create_recognition_rule::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateRecognitionRuleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        active: true,
        id: "example".to_string(),
        method: Default::default(),
        name: "example".to_string(),
    })
}
