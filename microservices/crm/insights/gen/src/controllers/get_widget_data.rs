// User-owned controller for handler 'get_widget_data'.

use crate::handlers::get_widget_data::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::NameCount;
#[allow(unused_imports)]
use crate::handlers::types::NamedSeries;

#[handler(GetWidgetDataController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        refreshed_at: "example".to_string(),
        rows: Some(vec![]),
        series: Some(vec![]),
        values: Some(vec![]),
        widget_id: "example".to_string(),
    })
}
