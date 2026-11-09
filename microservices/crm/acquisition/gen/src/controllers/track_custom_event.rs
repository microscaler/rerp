// User-owned controller for handler 'track_custom_event'.

use crate::handlers::track_custom_event::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(TrackCustomEventController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {})
}
