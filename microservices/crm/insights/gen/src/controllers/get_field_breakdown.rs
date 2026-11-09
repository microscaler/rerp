// User-owned controller for handler 'get_field_breakdown'.

use crate::handlers::get_field_breakdown::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::NameCount;

#[handler(GetFieldBreakdownController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        blank: Some(42),
        entity: "example".to_string(),
        field_id: "example".to_string(),
        field_label: Some("example".to_string()),
        values: vec![],
    })
}
