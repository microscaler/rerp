// User-owned controller for handler 'create_chart_template'.

use crate::handlers::create_chart_template::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateChartTemplateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        code: "example".to_string(),
        company_id: "example".to_string(),
        created_at: Some("example".to_string()),
        created_by: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        description: Some("example".to_string()),
        fiscal_day: Some(42),
        fiscal_month: Some(42),
        id: "example".to_string(),
        include_subperiods: Some(true),
        is_active: true,
        is_default: true,
        jurisdiction_code: "example".to_string(),
        name: "example".to_string(),
        number_of_periods: Some(42),
        status: "example".to_string(),
        updated_at: Some("example".to_string()),
        updated_by: Some("example".to_string()),
        version: "example".to_string(),
    }
}
