// User-owned controller for handler 'update_budget'.

use crate::handlers::update_budget::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateBudgetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        approval_status: Some("example".to_string()),
        approved_amount: Some(3.14),
        approved_at: Some("example".to_string()),
        approved_by: Some("example".to_string()),
        company_id: "example".to_string(),
        cost_center_id: Some("example".to_string()),
        created_at: Some("example".to_string()),
        created_by: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        department_id: Some("example".to_string()),
        description: "example".to_string(),
        fiscal_year: 42,
        id: "example".to_string(),
        name: "example".to_string(),
        notes: Some("example".to_string()),
        period_type: Some("example".to_string()),
        status: "example".to_string(),
        submitted_at: Some("example".to_string()),
        total_amount: Some(3.14),
        updated_at: Some("example".to_string()),
        version: Some(42),
    })
}
