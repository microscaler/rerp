// User-owned controller for handler 'list_chart_templates'.

use crate::handlers::list_chart_templates::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ChartTemplate;

#[handler(ListChartTemplatesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        has_more: Some(true),
        items: vec![],
        limit: 42,
        page: 42,
        total: 42,
    })
}
