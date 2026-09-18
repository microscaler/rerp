// User-owned controller for handler 'list_cash_transfers'.

use crate::handlers::list_cash_transfers::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListCashTransfersController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
