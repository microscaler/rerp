// User-owned controller for handler 'export_payment_batch_file'.

use crate::handlers::export_payment_batch_file::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ExportPaymentBatchFileController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        artifact_uri: Some("example".to_string()),
        batch_id: "example".to_string(),
        id: "example".to_string(),
        status: "example".to_string(),
    })
}
