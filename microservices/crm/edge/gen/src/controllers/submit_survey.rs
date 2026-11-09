// User-owned controller for handler 'submit_survey'.

use crate::handlers::submit_survey::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(SubmitSurveyController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        id: "example".to_string(),
        lead_id: Some("example".to_string()),
        respondent_email: "example".to_string(),
        respondent_name: Some("example".to_string()),
        response_date: Some("example".to_string()),
        responses: Some(Default::default()),
        survey_name: "example".to_string(),
    })
}
