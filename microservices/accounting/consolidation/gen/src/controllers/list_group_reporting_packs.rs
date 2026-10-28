// User-owned controller for handler 'list_group_reporting_packs'.

use crate::handlers::list_group_reporting_packs::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::GroupReportingPack;

#[handler(ListGroupReportingPacksController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: Some(vec![]),
        limit: 42,
        page: 42,
        total: 42,
    }
}
