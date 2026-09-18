// User-owned controller for handler 'create_deferred_expense'.

use crate::handlers::create_deferred_expense::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateDeferredExpenseController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        balance: 3.14,
        id: "example".to_string(),
        item_type: "example".to_string(),
        schedule_id: "example".to_string(),
    }
}
