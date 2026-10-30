// BRRTRouter: user-owned

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_accounting_invoice_gen::handlers::list_render_outbox::Request;
use serde_json::Value;

#[handler(ListRenderOutboxController)]
pub fn handle(request: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    let context = match crate::http_support::identity_context(request.jwt_claims.as_ref()) {
        Ok(context) => context,
        Err(response) => return response,
    };
    let data = &request.data;
    let filter = match crate::outbox::OutboxListFilter::parse(
        data.status.as_deref(),
        data.document_id.as_deref(),
        data.page,
        data.limit,
    ) {
        Ok(filter) => filter,
        Err(error) => return crate::http_support::posting_error(error),
    };
    match crate::http_support::with_accounting_transaction(&context, |executor| {
        crate::outbox::list_outbox(executor, &context, &filter)
    }) {
        Ok(page) => HttpJson::new(200, page),
        Err(error) => crate::http_support::posting_error(error),
    }
}
