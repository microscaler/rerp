// User-owned controller for handler 'list_invoice_payment_matches'.

use crate::handlers::list_invoice_payment_matches::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::InvoicePaymentMatch;

#[handler(ListInvoicePaymentMatchesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        items: vec![],
        total: 42,
    }
}
