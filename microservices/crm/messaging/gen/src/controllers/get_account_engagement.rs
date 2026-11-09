// User-owned controller for handler 'get_account_engagement'.

use crate::handlers::get_account_engagement::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetAccountEngagementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        account_id: "example".to_string(),
        points: vec![],
    })
}
