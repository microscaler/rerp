// User-owned controller for handler 'delete_bank_statement'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::delete_bank_statement::{Request, Response};

#[handler(DeleteBankStatementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
