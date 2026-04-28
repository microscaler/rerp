// User-owned controller for handler 'create_budget_revision'.

use crate::handlers::create_budget_revision::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateBudgetRevisionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        budget_id: "example".to_string(),
        id: "example".to_string(),
        reason: Some("example".to_string()),
        requested_by: Some("example".to_string()),
        revision_number: 42,
        status: "example".to_string(),
    }
}
