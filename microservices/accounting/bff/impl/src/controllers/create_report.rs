// User-owned controller for handler 'create_report'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_bff_gen::handlers::create_report::{Request, Response};

#[handler(CreateReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        approved_at: Some("example".to_string()),
        approved_by: Some("example".to_string()),
        company_id: Some("example".to_string()),
        created_at: Some("example".to_string()),
        created_by: Some("example".to_string()),
        currency_code: "example".to_string(),
        description: Some("example".to_string()),
        generated_at: Some("example".to_string()),
        generated_by: Some("example".to_string()),
        id: "example".to_string(),
        metadata: Some(Default::default()),
        name: "example".to_string(),
        parameters: Some(Default::default()),
        period_end: Some("example".to_string()),
        period_start: Some("example".to_string()),
        report_code: "example".to_string(),
        report_data: Some(Default::default()),
        report_date: Some("example".to_string()),
        report_type: "example".to_string(),
        status: "example".to_string(),
        template_id: Some("example".to_string()),
        updated_at: Some("example".to_string()),
        updated_by: Some("example".to_string()),
    }
}
