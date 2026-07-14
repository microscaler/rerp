// User-owned controller for handler 'post_customer_invoice'.

use crate::handlers::post_customer_invoice::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::PostedInvoice;
#[allow(unused_imports)]
use crate::handlers::types::PostedJournal;

#[handler(PostCustomerInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        idempotency_key: "example".to_string(),
        invoice: Default::default(),
        journal: Default::default(),
        request_fingerprint: "example".to_string(),
    }
}
