// User-owned controller for handler 'create_liquidity_plan'.

use crate::handlers::create_liquidity_plan::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateLiquidityPlanController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        company_id: "example".to_string(),
        id: "example".to_string(),
        minimum_cash_threshold: Some(3.14),
        plan_date: "example".to_string(),
        status: Default::default(),
    }
}
