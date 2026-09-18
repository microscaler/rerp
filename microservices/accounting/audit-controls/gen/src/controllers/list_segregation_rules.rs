// User-owned controller for handler 'list_segregation_rules'.

use crate::handlers::list_segregation_rules::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListSegregationRulesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
