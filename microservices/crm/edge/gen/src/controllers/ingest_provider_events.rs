// User-owned controller for handler 'ingest_provider_events'.

use crate::handlers::ingest_provider_events::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(IngestProviderEventsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        code: 42,
        details: Some("example".to_string()),
        fields: Some(vec![]),
        message: "example".to_string(),
        request_id: Some("example".to_string()),
    })
}
