// User-owned controller for handler 'submit_form'.

use crate::handlers::submit_form::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(SubmitFormController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "message": "Example",
    //   "status": "CONFIRMATION_SENT"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "message": "Example",
  "status": "CONFIRMATION_SENT"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        message: Some("Example".to_string()),
        status: "CONFIRMATION_SENT".to_string(),
    })
}
