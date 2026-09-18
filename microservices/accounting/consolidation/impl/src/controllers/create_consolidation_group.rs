// Implementation stub for handler 'create_consolidation_group'
// This file is a starting point for your implementation.
// You can modify this file freely - it will NOT be auto-regenerated.

use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_consolidation_gen::handlers::create_consolidation_group::{Request, Response};

#[handler(CreateConsolidationGroupController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // TODO: Implement your business logic here
    Response {
        id: "".to_string(),
        name: "".to_string(),
        parent_company_id: None,
        reporting_currency_code: "".to_string(),
    }
}
