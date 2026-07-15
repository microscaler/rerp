// User-owned controller for handler 'create_edi_submission'.

use crate::handlers::create_edi_submission::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::EdiSubmissionStatus;

#[handler(CreateEdiSubmissionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        document_id: "example".to_string(),
        external_reference: Some("example".to_string()),
        id: "example".to_string(),
        profile_id: "example".to_string(),
        status: Default::default(),
        submitted_at: Some("example".to_string()),
    })
}
