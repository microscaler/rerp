// User-owned controller for handler 'list_recognition_schedules'.

use crate::handlers::list_recognition_schedules::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListRecognitionSchedulesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
