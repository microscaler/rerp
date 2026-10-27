// User-owned controller for handler 'reconciliation_report'.

use crate::handlers::reconciliation_report::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ReconciliationReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        bank_account_id: "example".to_string(),
        bank_account_name: Some("example".to_string()),
        closing_balance: Some(1.5),
        created_at: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        id: Some("example".to_string()),
        opening_balance: Some(1.5),
        period_end: "example".to_string(),
        period_start: "example".to_string(),
        reconciled_amount: Some(1.5),
        reconciliation_percentage: Some(1.5),
        total_deposits: Some(1.5),
        total_withdrawals: Some(1.5),
        unreconciled_amount: Some(1.5),
    })
}
