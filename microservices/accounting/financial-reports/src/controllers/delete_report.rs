// User-owned controller for handler 'delete_report'.
use crate::handlers::delete_report::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
