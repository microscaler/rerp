// User-owned controller for handler 'list_liquidity_plans'.

use crate::handlers::list_liquidity_plans::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListLiquidityPlansController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
