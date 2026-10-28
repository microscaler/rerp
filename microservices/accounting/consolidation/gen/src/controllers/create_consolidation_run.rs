// User-owned controller for handler 'create_consolidation_run'.

use crate::handlers::create_consolidation_run::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateConsolidationRunController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        executed_at: Some("example".to_string()),
        fiscal_period_id: "example".to_string(),
        group_id: "example".to_string(),
        id: "example".to_string(),
        status: Default::default(),
    })
}
