// User-owned controller for handler 'delete_bank_statement'.
use crate::handlers::delete_bank_statement::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteBankStatementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
