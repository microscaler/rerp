// User-owned controller for handler 'get_social_calendar'.

use crate::handlers::get_social_calendar::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetSocialCalendarController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response { days: vec![] })
}
