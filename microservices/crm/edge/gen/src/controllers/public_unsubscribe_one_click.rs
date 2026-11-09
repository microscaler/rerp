// User-owned controller for handler 'public_unsubscribe_one_click'.

use crate::handlers::public_unsubscribe_one_click::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(PublicUnsubscribeOneClickController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {})
}
