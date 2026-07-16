// User-owned controller for handler 'get_customer_invoice_document'.

use crate::handlers::get_customer_invoice_document::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetCustomerInvoiceDocumentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        document_id: "example".to_string(),
        download_url: "example".to_string(),
        expires_at: "example".to_string(),
        media_type: "example".to_string(),
        rendered_at: "example".to_string(),
        renderer: "example".to_string(),
        renderer_version: "example".to_string(),
        sha256: "example".to_string(),
        size_bytes: 42,
    }
}
