// User-owned controller for handler 'list_recognition_rules'.

use crate::handlers::list_recognition_rules::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListRecognitionRulesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
