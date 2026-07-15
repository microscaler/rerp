// User-owned controller for handler 'create_follow_up_run'.

use crate::handlers::create_follow_up_run::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateFollowUpRunController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        company_id: Some("example".to_string()),
        customer_count: Some(42),
        id: "example".to_string(),
        status: "example".to_string(),
    })
}
