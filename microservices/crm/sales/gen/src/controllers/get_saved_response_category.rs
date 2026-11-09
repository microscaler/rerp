// User-owned controller for handler 'get_saved_response_category'.

use crate::handlers::get_saved_response_category::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetSavedResponseCategoryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        id: "example".to_string(),
        name: "example".to_string(),
        response_count: Some(42),
    })
}
