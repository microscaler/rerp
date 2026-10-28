// User-owned controller for handler 'generate_income_statement'.

use crate::handlers::generate_income_statement::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GenerateIncomeStatementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        account_details: Some(vec![]),
        company_id: "example".to_string(),
        cost_of_goods_sold: Some(1.5),
        created_at: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        gross_profit: Some(1.5),
        id: Some("example".to_string()),
        net_income: 1.5,
        net_margin: Some(1.5),
        operating_expenses: Some(Default::default()),
        operating_income: Some(1.5),
        other_income_expense: Some(1.5),
        period_end: "example".to_string(),
        period_start: "example".to_string(),
        revenue: Some(Default::default()),
        tax_expense: Some(1.5),
    })
}
