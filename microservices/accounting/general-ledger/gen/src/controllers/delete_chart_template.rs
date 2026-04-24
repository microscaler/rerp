// User-owned controller for handler 'delete_chart_template'.

use crate::handlers::delete_chart_template::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeleteChartTemplateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        code: "example".to_string(),
        details: Some(Default::default()),
        message: "example".to_string(),
    }
}
