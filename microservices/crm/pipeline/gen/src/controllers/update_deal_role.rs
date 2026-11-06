// User-owned controller for handler 'update_deal_role'.

use crate::handlers::update_deal_role::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateDealRoleController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        description: Some("example".to_string()),
        id: "example".to_string(),
        title: "example".to_string(),
        used_on: Some(42),
    })
}
