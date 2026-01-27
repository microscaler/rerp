// User-owned controller for handler 'delete_chart_of_account'.
use crate::handlers::delete_chart_of_account::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteChartOfAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
