// User-owned controller for handler 'list_approvals'.

use crate::handlers::list_approvals::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ApprovalRecord;

#[handler(ListApprovalsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        has_more: Some(true),
        items: vec![],
        limit: 42,
        page: 42,
        total: 42,
    })
}
