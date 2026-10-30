// User-owned controller for handler 'create_fiscal_period'.

use crate::handlers::create_fiscal_period::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateFiscalPeriodController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        closed_at: Some("example".to_string()),
        closed_by: Some("example".to_string()),
        end_date: "example".to_string(),
        id: "example".to_string(),
        name: "example".to_string(),
        start_date: "example".to_string(),
        state: "example".to_string(),
    })
}
