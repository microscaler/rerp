// User-owned controller for handler 'handoff_invoice_to_einvoice'.

use crate::handlers::handoff_invoice_to_einvoice::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(HandoffInvoiceToEinvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        external_reference: Some("example".to_string()),
        id: "example".to_string(),
        invoice_id: "example".to_string(),
        status: "example".to_string(),
        target_service: "example".to_string(),
    })
}
