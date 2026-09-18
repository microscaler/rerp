// User-owned controller for handler 'list_recognition_runs'.

use crate::handlers::list_recognition_runs::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListRecognitionRunsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
