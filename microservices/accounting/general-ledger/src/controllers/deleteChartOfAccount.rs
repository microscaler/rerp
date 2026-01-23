// User-owned controller for handler 'deleteChartOfAccount'.
use crate::handlers::deleteChartOfAccount::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteChartOfAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
