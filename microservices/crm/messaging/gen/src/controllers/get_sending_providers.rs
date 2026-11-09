// User-owned controller for handler 'get_sending_providers'.

use crate::handlers::get_sending_providers::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetSendingProvidersController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        channels: Some(vec![]),
        email: Some(Default::default()),
    })
}
