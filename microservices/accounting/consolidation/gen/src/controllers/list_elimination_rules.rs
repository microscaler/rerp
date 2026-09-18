// User-owned controller for handler 'list_elimination_rules'.

use crate::handlers::list_elimination_rules::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListEliminationRulesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
