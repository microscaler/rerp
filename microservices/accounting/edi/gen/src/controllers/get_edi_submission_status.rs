// User-owned controller for handler 'get_edi_submission_status'.

use crate::handlers::get_edi_submission_status::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::EdiSubmissionStatus;

#[handler(GetEdiSubmissionStatusController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        document_id: "example".to_string(),
        external_reference: Some("example".to_string()),
        id: "example".to_string(),
        profile_id: "example".to_string(),
        status: Default::default(),
        submitted_at: Some("example".to_string()),
    }
}
