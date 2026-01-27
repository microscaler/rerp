// User-owned controller for handler 'delete_bank_account'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bank_sync_gen::handlers::delete_bank_account::{Request, Response};

#[handler(DeleteBankAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
