// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_invoice_gen::handlers::get_customer_invoice::Request;
use serde_json::Value;

#[handler(GetCustomerInvoiceController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    let document_id = match crate::posting::parse_uuid(&request.data.id, "id") {
        Ok(id) => id,
        Err(error) => return crate::http_support::posting_error(error),
    };
    match crate::http_support::with_accounting_transaction(&context, |executor| {
        crate::posting::get_invoice(executor, document_id)
    }) {
        Ok(invoice) => HttpJson::new(200, crate::posting::invoice_json(&invoice)),
        Err(error) => crate::http_support::posting_error(error),
    }
}
