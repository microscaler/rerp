// User-owned controller for handler 'list_group_reporting_packs'.

use crate::handlers::list_group_reporting_packs::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ListGroupReportingPacksController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
