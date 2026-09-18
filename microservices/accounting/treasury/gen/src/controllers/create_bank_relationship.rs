// User-owned controller for handler 'create_bank_relationship'.

use crate::handlers::create_bank_relationship::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateBankRelationshipController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        active: true,
        bank_name: "example".to_string(),
        id: "example".to_string(),
        relationship_type: "example".to_string(),
    }
}
