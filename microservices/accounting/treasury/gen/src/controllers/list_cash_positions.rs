// User-owned controller for handler 'list_cash_positions'.

use crate::handlers::list_cash_positions::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListCashPositionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
