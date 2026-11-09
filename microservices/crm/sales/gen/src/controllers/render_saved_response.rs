// User-owned controller for handler 'render_saved_response'.

use crate::handlers::render_saved_response::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RenderSavedResponseController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "body": "Hi %FIRSTNAME%,\n\nYou're on day %TRIAL_DAY% of your trial of %PRODUCT%. I noticed %ACCOUNT% hasn't set up %FEATURE% yet — happy to walk you through it in 15 minutes.\n\n%SIGNATURE%",
    //   "subject": "How is your %FIRSTNAME% trial going?"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "body": "Hi %FIRSTNAME%,\n\nYou're on day %TRIAL_DAY% of your trial of %PRODUCT%. I noticed %ACCOUNT% hasn't set up %FEATURE% yet — happy to walk you through it in 15 minutes.\n\n%SIGNATURE%",
  "subject": "How is your %FIRSTNAME% trial going?"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        body: Some("Hi %FIRSTNAME%,\n\nYou're on day %TRIAL_DAY% of your trial of %PRODUCT%. I noticed %ACCOUNT% hasn't set up %FEATURE% yet — happy to walk you through it in 15 minutes.\n\n%SIGNATURE%".to_string()),subject: Some("How is your %FIRSTNAME% trial going?".to_string()),
    })
}
