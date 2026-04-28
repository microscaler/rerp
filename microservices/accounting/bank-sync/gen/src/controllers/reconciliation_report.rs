// User-owned controller for handler 'reconciliation_report'.

use crate::handlers::reconciliation_report::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ReconciliationReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        bank_account_id: "example".to_string(),
        bank_account_name: Some("example".to_string()),
        closing_balance: Some(3.14),
        created_at: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        id: Some("example".to_string()),
        opening_balance: Some(3.14),
        period_end: "example".to_string(),
        period_start: "example".to_string(),
        reconciled_amount: Some(3.14),
        reconciliation_percentage: Some(3.14),
        total_deposits: Some(3.14),
        total_withdrawals: Some(3.14),
        unreconciled_amount: Some(3.14),
    }
}
