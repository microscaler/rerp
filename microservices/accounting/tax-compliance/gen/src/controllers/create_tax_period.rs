// User-owned controller for handler 'create_tax_period'.

use crate::handlers::create_tax_period::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateTaxPeriodController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        company_id: "example".to_string(),
        due_date: Some("example".to_string()),
        end_date: "example".to_string(),
        id: "example".to_string(),
        jurisdiction_code: "example".to_string(),
        start_date: "example".to_string(),
        status: Default::default(),
        tax_type: Default::default(),
    }
}
