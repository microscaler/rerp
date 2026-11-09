// User-owned controller for handler 'get_automation_catalog'.

use crate::handlers::get_automation_catalog::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::CatalogEntry;

#[handler(GetAutomationCatalogController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        actions: vec![],
        triggers: vec![],
    })
}
