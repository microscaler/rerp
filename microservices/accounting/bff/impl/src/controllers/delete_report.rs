// User-owned controller for handler 'delete_report'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::delete_report::{Request, Response};

#[handler(DeleteReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
