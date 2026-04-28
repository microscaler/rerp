// User-owned controller for handler 'create_tax_return'.

use crate::handlers::create_tax_return::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateTaxReturnController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        company_id: "example".to_string(),
        currency_code: Some("example".to_string()),
        id: "example".to_string(),
        period_id: "example".to_string(),
        return_type: "example".to_string(),
        status: Default::default(),
        submitted_at: Some("example".to_string()),
        total_due: 3.14,
    }
}
