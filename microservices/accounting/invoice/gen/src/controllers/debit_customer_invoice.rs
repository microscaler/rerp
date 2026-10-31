// User-owned controller for handler 'debit_customer_invoice'.

use crate::handlers::debit_customer_invoice::{ApiResponse, Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::PostedInvoice;
#[allow(unused_imports)]
use crate::handlers::types::PostedJournal;

#[handler(DebitCustomerInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> ApiResponse {
    ApiResponse::Ok(Response {
        idempotency_key: "example".to_string(),
        invoice: Default::default(),
        journal: Default::default(),
        request_fingerprint: "example".to_string(),
    })
}
