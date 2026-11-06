// User-owned controller for handler 'get_deal_role'.

use crate::handlers::get_deal_role::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetDealRoleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        description: Some("example".to_string()),
        id: "example".to_string(),
        title: "example".to_string(),
        used_on: Some(42),
    })
}
