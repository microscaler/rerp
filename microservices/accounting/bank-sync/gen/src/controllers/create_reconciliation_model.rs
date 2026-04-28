// User-owned controller for handler 'create_reconciliation_model'.

use crate::handlers::create_reconciliation_model::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateReconciliationModelController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        active: true,
        created_at: Some("example".to_string()),
        id: "example".to_string(),
        match_tolerance_amount: Some(3.14),
        match_tolerance_days: Some(42),
        name: "example".to_string(),
        rule_type: "example".to_string(),
        sequence: Some(42),
        updated_at: Some("example".to_string()),
    }
}
