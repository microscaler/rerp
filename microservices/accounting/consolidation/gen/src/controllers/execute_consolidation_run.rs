// User-owned controller for handler 'execute_consolidation_run'.

use crate::handlers::execute_consolidation_run::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ExecuteConsolidationRunController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        executed_at: Some("example".to_string()),
        fiscal_period_id: "example".to_string(),
        group_id: "example".to_string(),
        id: "example".to_string(),
        status: Default::default(),
    })
}
