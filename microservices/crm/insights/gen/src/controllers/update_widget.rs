// User-owned controller for handler 'update_widget'.

use crate::handlers::update_widget::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ReportSchedule;
#[allow(unused_imports)]
use crate::handlers::types::WidgetQuery;

#[handler(UpdateWidgetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        id: "example".to_string(),
        kind: "example".to_string(),
        position: Some(42),
        prompt: Some("example".to_string()),
        query: Default::default(),
        refreshed_at: Some("example".to_string()),
        schedule: Some(Default::default()),
        title: "example".to_string(),
        width: Some(42),
    })
}
