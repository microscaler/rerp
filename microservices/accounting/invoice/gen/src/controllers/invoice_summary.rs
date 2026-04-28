// User-owned controller for handler 'invoice_summary'.

use crate::handlers::invoice_summary::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(InvoiceSummaryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        average_invoice_amount: Some(3.14),
        by_status: Default::default(),
        by_type: Default::default(),
        company_id: Some("example".to_string()),
        currency_code: Some("example".to_string()),
        period_end: Some("example".to_string()),
        period_start: Some("example".to_string()),
        tax_collected: Some(3.14),
        total_amount: 3.14,
        total_invoices: 42,
    }
}
