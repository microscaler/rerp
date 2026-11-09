// User-owned controller for handler 'recalculate_score'.

use crate::handlers::recalculate_score::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RecalculateScoreController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "score_id": "9e979219-1db4-56a0-b42a-37095e08f504",
    //   "status": "QUEUED"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "score_id": "9e979219-1db4-56a0-b42a-37095e08f504",
  "status": "QUEUED"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        queued_at: Some("example".to_string()),
        score_id: "9e979219-1db4-56a0-b42a-37095e08f504".to_string(),
        status: "QUEUED".to_string(),
    })
}
