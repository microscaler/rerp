// User-owned controller for handler 'create_consolidation_group'.

use crate::handlers::create_consolidation_group::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateConsolidationGroupController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        id: "example".to_string(),
        name: "example".to_string(),
        parent_company_id: Some("example".to_string()),
        reporting_currency_code: "example".to_string(),
    }
}
