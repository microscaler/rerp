// User-owned controller for handler 'public_preferences_update'.

use crate::handlers::public_preferences_update::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(PublicPreferencesUpdateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        email_masked: "example".to_string(),
        lists: vec![],
    })
}
