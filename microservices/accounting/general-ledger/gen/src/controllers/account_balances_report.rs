// User-owned controller for handler 'account_balances_report'.

use crate::handlers::account_balances_report::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AccountBalance;

#[handler(AccountBalancesReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        company_id: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        lines: Some(vec![]),
        net_income: Some(3.14),
        report_date: Some("example".to_string()),
        total_assets: Some(3.14),
        total_equity: Some(3.14),
        total_expenses: Some(3.14),
        total_liabilities: Some(3.14),
        total_revenue: Some(3.14),
    })
}
