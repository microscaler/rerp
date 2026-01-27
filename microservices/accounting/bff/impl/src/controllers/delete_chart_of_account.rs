// User-owned controller for handler 'delete_chart_of_account'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::delete_chart_of_account::{Request, Response};

#[handler(DeleteChartOfAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
