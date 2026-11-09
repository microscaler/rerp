// User-owned controller for handler 'get_saved_response'.

use crate::handlers::get_saved_response::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetSavedResponseController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "body": "Hi %FIRSTNAME%,\n\nYou're on day %TRIAL_DAY% of your trial of %PRODUCT%. I noticed %ACCOUNT% hasn't set up %FEATURE% yet — happy to walk you through it in 15 minutes.\n\n%SIGNATURE%",
    //   "category": "Sales",
    //   "category_id": "113ca5aa-e3f2-576a-b6b5-79dbb715308e",
    //   "id": "9587a888-ee0e-50ac-8e1a-38a273fccd36",
    //   "name": "Northwind Traders",
    //   "subject": "How is your %FIRSTNAME% trial going?",
    //   "title": "Trial follow-up",
    //   "updated_at": "2026-11-04T16:00:00Z",
    //   "use_count": 41
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "body": "Hi %FIRSTNAME%,\n\nYou're on day %TRIAL_DAY% of your trial of %PRODUCT%. I noticed %ACCOUNT% hasn't set up %FEATURE% yet — happy to walk you through it in 15 minutes.\n\n%SIGNATURE%",
  "category": "Sales",
  "category_id": "113ca5aa-e3f2-576a-b6b5-79dbb715308e",
  "id": "9587a888-ee0e-50ac-8e1a-38a273fccd36",
  "name": "Northwind Traders",
  "subject": "How is your %FIRSTNAME% trial going?",
  "title": "Trial follow-up",
  "updated_at": "2026-11-04T16:00:00Z",
  "use_count": 41
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        body: "Hi %FIRSTNAME%,\n\nYou're on day %TRIAL_DAY% of your trial of %PRODUCT%. I noticed %ACCOUNT% hasn't set up %FEATURE% yet — happy to walk you through it in 15 minutes.\n\n%SIGNATURE%".to_string(),category: Some("Sales".to_string()),category_id: Some("113ca5aa-e3f2-576a-b6b5-79dbb715308e".to_string()),created_at: Some("example".to_string()),created_by: Some("example".to_string()),id: "9587a888-ee0e-50ac-8e1a-38a273fccd36".to_string(),last_sent_at: Some("example".to_string()),last_sent_by: Some("example".to_string()),name: "Northwind Traders".to_string(),send_count: Some(42),subject: "How is your %FIRSTNAME% trial going?".to_string(),title: Some("Trial follow-up".to_string()),updated_at: Some("2026-11-04T16:00:00Z".to_string()),use_count: Some(41),
    })
}
