// User-owned controller for handler 'get_account_contacts'.

use crate::handlers::get_account_contacts::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetAccountContactsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        limit: 42,
        page: 42,
        total: 42,
    }
}
