// User-owned controller for handler 'delete_financial_statement'.
use crate::handlers::delete_financial_statement::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteFinancialStatementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
