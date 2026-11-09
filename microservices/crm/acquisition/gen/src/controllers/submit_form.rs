// User-owned controller for handler 'submit_form'.

use crate::handlers::submit_form::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(SubmitFormController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        contact_id: "example".to_string(),
        form_id: "example".to_string(),
        lead_id: "example".to_string(),
        redirect_url: "example".to_string(),
    })
}
