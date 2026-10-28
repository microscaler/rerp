// User-owned controller for handler 'delete_bank_account'.

use crate::handlers::delete_bank_account::Request;
use brrtrouter::typed::HttpNoContent;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteBankAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpNoContent {
    HttpNoContent
}
