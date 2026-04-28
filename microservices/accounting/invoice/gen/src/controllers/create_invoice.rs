// User-owned controller for handler 'create_invoice'.

use crate::handlers::create_invoice::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        billing_address_id: Some("example".to_string()),
        billing_entity_id: Some("example".to_string()),
        company_currency_code: Some("example".to_string()),
        company_id: Some("example".to_string()),
        company_total_amount: Some(3.14),
        created_at: Some("example".to_string()),
        created_by: Some("example".to_string()),
        currency_code: "example".to_string(),
        due_date: Some("example".to_string()),
        entity_id: Some("example".to_string()),
        entity_name: Some("example".to_string()),
        exchange_rate: Some(3.14),
        id: "example".to_string(),
        internal_notes: Some("example".to_string()),
        invoice_number: "example".to_string(),
        invoice_type: "example".to_string(),
        issued_date: Some("example".to_string()),
        notes: Some("example".to_string()),
        posted_at: Some("example".to_string()),
        posted_by: Some("example".to_string()),
        shipping_address_id: Some("example".to_string()),
        status: "example".to_string(),
        subtotal_amount: Some(3.14),
        tax_amount: Some(3.14),
        total_amount: 3.14,
        updated_at: Some("example".to_string()),
    }
}
