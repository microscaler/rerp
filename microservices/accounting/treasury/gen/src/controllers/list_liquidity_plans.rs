// User-owned controller for handler 'list_liquidity_plans'.

use crate::handlers::list_liquidity_plans::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::LiquidityPlan;

#[handler(ListLiquidityPlansController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: Some(vec![]),
        limit: 42,
        page: 42,
        total: 42,
    }
}
