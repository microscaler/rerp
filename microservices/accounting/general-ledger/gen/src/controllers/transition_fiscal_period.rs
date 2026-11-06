// User-owned controller for handler 'transition_fiscal_period'.

use crate::handlers::transition_fiscal_period::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(TransitionFiscalPeriodController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        closed_at: Some(Default::default()),
        closed_by: Some(Default::default()),
        end_date: "example".to_string(),
        id: "example".to_string(),
        name: "example".to_string(),
        start_date: "example".to_string(),
        state: "example".to_string(),
    })
}
