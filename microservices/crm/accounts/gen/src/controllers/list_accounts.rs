// User-owned controller for handler 'list_accounts'.

use crate::handlers::list_accounts::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListAccountsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
