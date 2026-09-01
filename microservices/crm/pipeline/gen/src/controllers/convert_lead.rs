// User-owned controller for handler 'convert_lead'.

use crate::handlers::convert_lead::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ConvertLeadController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        account_id: "example".to_string(),
        contact_id: "example".to_string(),
        converted_lead: Some(Default::default()),
        lead_id: "example".to_string(),
    })
}
