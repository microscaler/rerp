// User-owned controller for handler 'delete_custom_report'.

use crate::handlers::delete_custom_report::Request;
use brrtrouter::typed::HttpNoContent;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteCustomReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpNoContent {
    HttpNoContent
}
