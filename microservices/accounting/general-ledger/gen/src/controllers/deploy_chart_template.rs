// User-owned controller for handler 'deploy_chart_template'.

use crate::handlers::deploy_chart_template::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(DeployChartTemplateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        account_count: 42,
        chart_of_account_id: "example".to_string(),
        chart_template_id: Some("example".to_string()),
        created_at: Some("example".to_string()),
        created_periods: Some(42),
        fiscal_year_id: "example".to_string(),
        id: Some("example".to_string()),
    })
}
