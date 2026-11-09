// User-owned controller for handler 'submit_survey'.

use crate::handlers::submit_survey::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(SubmitSurveyController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "id": "9fa51332-6221-502c-a499-80ae6ddde550",
    //   "lead_id": "b68dbf73-2479-5234-b517-798dd1265706",
    //   "respondent_email": "Respondent email",
    //   "respondent_name": "Respondent name",
    //   "response_date": "2026-11-05T16:00:00Z",
    //   "responses": {},
    //   "survey_name": "Survey name"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "id": "9fa51332-6221-502c-a499-80ae6ddde550",
  "lead_id": "b68dbf73-2479-5234-b517-798dd1265706",
  "respondent_email": "Respondent email",
  "respondent_name": "Respondent name",
  "response_date": "2026-11-05T16:00:00Z",
  "responses": {},
  "survey_name": "Survey name"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        id: "9fa51332-6221-502c-a499-80ae6ddde550".to_string(),
        lead_id: Some("b68dbf73-2479-5234-b517-798dd1265706".to_string()),
        respondent_email: "Respondent email".to_string(),
        respondent_name: Some("Respondent name".to_string()),
        response_date: Some("2026-11-05T16:00:00Z".to_string()),
        responses: Some(
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),
        ),
        survey_name: "Survey name".to_string(),
    })
}
