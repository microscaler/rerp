// User-owned controller for handler 'list_reconciliation_models'.

use crate::handlers::list_reconciliation_models::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ReconciliationModel;

#[handler(ListReconciliationModelsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: vec![],
        total: 42,
    }
}
