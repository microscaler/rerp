// User-owned controller for handler 'get_lead_source_effectiveness'.

use crate::handlers::get_lead_source_effectiveness::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::LeadSourceReport;

#[handler(GetLeadSourceEffectivenessController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        items: vec![],
        limit: 42,
        page: 42,
        total: 42,
    })
}
