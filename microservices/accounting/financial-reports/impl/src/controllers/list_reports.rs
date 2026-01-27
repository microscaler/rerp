// User-owned controller for handler 'list_reports'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_financial_reports_gen::handlers::list_reports::{Request, Response};

#[allow(unused_imports)]
use rerp_accounting_financial_reports_gen::handlers::types::Report;

#[handler(ListReportsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: Some(Default::default()),
        limit: Some(42),
        page: Some(42),
        total: Some(42),
    }
}
