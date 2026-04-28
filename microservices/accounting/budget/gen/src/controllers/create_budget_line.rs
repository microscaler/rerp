// User-owned controller for handler 'create_budget_line'.

use crate::handlers::create_budget_line::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateBudgetLineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        actual_amount: Some(3.14),
        amount: 3.14,
        budget_id: "example".to_string(),
        cost_center_id: Some("example".to_string()),
        created_at: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        department_id: Some("example".to_string()),
        gl_account_code: Some("example".to_string()),
        gl_account_id: Some("example".to_string()),
        gl_account_name: Some("example".to_string()),
        id: "example".to_string(),
        notes: Some("example".to_string()),
        period: "example".to_string(),
        period_name: Some("example".to_string()),
        updated_at: Some("example".to_string()),
        variance: Some(3.14),
        variance_percent: Some(3.14),
    }
}
