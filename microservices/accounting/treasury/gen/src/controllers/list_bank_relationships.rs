// User-owned controller for handler 'list_bank_relationships'.

use crate::handlers::list_bank_relationships::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListBankRelationshipsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
