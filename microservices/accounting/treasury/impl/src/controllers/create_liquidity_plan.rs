// Implementation stub for handler 'create_liquidity_plan'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_treasury_gen::handlers::create_liquidity_plan::{Request, Response};

#[handler(CreateLiquidityPlanController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        company_id: "".to_string(),
        id: "".to_string(),
        minimum_cash_threshold: None,
        plan_date: "".to_string(),
        status: serde_json::json!({}),
    }
}
