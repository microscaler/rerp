// User-owned controller for handler 'get_transaction_reconciliation_suggestions'.

use crate::handlers::get_transaction_reconciliation_suggestions::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ReconciliationSuggestion;

#[handler(GetTransactionReconciliationSuggestionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        suggestions: vec![],
        transaction_id: "example".to_string(),
    }
}
