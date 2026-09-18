// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_invoice_gen::handlers::post_customer_invoice::Request;
use serde_json::Value;

#[handler(PostCustomerInvoiceController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    match crate::http_support::with_accounting_transaction(&context, |executor| {
        crate::posting::post_customer_invoice(executor, &context, &request.data)
    }) {
        Ok(posting) => HttpJson::new(
            if posting.created { 201 } else { 200 },
            crate::posting::posting_json(&posting),
        ),
        Err(error) => crate::http_support::posting_error(error),
    }
}
