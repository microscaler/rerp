// User-owned controller for handler 'enrol_contacts'.

use crate::handlers::enrol_contacts::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(EnrolContactsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        already_active: 42,
        enrolled: 42,
        enrolment_ids: Some(vec![]),
        excluded: 42,
        skipped: Some(vec![]),
    })
}
