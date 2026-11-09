// User-owned controller for handler 'get_saved_report'.

use crate::handlers::get_saved_report::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetSavedReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        created_at: Some("example".to_string()),
        data: Some(Default::default()),
        filters: Some(Default::default()),
        id: Some("example".to_string()),
        name: "example".to_string(),
        r#type: "example".to_string(),
    })
}
