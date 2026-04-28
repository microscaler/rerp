// User-owned controller for handler 'create_edi_profile'.

use crate::handlers::create_edi_profile::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::EdiStandard;

#[handler(CreateEdiProfileController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        active: true,
        id: "example".to_string(),
        jurisdiction_code: Some("example".to_string()),
        name: "example".to_string(),
        standard: Default::default(),
        trading_partner_id: Some("example".to_string()),
    }
}
