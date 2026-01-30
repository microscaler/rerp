// User-owned controller for handler 'get_financial_statement'.
use crate::handlers::get_financial_statement::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetFinancialStatementController)]
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
        net_income: Some(rust_decimal::Decimal::new(12345, 2)),
        report_date: "example".to_string(),
        report_id: "example".to_string(),
        summary: Some(Default::default()),
        total_assets: Some(rust_decimal::Decimal::new(12345, 2)),
        total_liabilities: Some(rust_decimal::Decimal::new(12345, 2)),
        updated_at: Some("example".to_string()),
    }
}
