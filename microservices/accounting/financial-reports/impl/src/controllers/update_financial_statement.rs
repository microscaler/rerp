// User-owned controller for handler 'update_financial_statement'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_financial_reports_gen::handlers::update_financial_statement::{
    Request, Response,
};

#[handler(UpdateFinancialStatementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        company_id: Some("example".to_string()),
        created_at: Some("example".to_string()),
        currency_code: "example".to_string(),
        data: Default::default(),
        data_version: 42,
        generated_at: Some("example".to_string()),
        generated_by: Some("example".to_string()),
        id: "example".to_string(),
        metadata: Some(Default::default()),
        net_income: None,
        report_date: "example".to_string(),
        report_id: "example".to_string(),
        summary: Some(Default::default()),
        total_assets: None,
        total_liabilities: None,
        updated_at: Some("example".to_string()),
    }
}
