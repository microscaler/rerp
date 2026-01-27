// User-owned controller for handler 'delete_payment'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_accounts_receivable_gen::handlers::delete_payment::{Request, Response};

#[handler(DeletePaymentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
