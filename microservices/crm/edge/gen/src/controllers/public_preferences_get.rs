// User-owned controller for handler 'public_preferences_get'.

use crate::handlers::public_preferences_get::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(PublicPreferencesGetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        email_masked: "example".to_string(),
        lists: vec![],
    })
}
