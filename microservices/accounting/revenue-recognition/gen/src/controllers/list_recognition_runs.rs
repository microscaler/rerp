// User-owned controller for handler 'list_recognition_runs'.

use crate::handlers::list_recognition_runs::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::RecognitionRun;

#[handler(ListRecognitionRunsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: Some(vec![]),
        limit: 42,
        page: 42,
        total: 42,
    }
}
