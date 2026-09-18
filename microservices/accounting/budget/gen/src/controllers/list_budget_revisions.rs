// User-owned controller for handler 'list_budget_revisions'.

use crate::handlers::list_budget_revisions::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::BudgetRevision;

#[handler(ListBudgetRevisionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: vec![],
        total: 42,
    }
}
