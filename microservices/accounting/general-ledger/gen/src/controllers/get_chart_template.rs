// User-owned controller for handler 'get_chart_template'.

use crate::handlers::get_chart_template::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetChartTemplateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {}
}
