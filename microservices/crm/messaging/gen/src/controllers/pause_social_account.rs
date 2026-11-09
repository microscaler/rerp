// User-owned controller for handler 'pause_social_account'.

use crate::handlers::pause_social_account::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Platform;

#[handler(PauseSocialAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "created_at": "2026-07-08T16:00:00Z",
    //   "display_name": "Acme Co",
    //   "followers": 12400,
    //   "handle": "@acme",
    //   "id": "a41279e1-eaf5-5755-8860-1b6c8aebc4e3",
    //   "paused_reason": "Paused reason",
    //   "platform": "X",
    //   "status": "CONNECTED"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "created_at": "2026-07-08T16:00:00Z",
  "display_name": "Acme Co",
  "followers": 12400,
  "handle": "@acme",
  "id": "a41279e1-eaf5-5755-8860-1b6c8aebc4e3",
  "paused_reason": "Paused reason",
  "platform": "X",
  "status": "CONNECTED"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        authorization_url: Some("example".to_string()),
        avatar_url: Some("example".to_string()),
        created_at: "2026-07-08T16:00:00Z".to_string(),
        display_name: Some("Acme Co".to_string()),
        followers: Some(12400),
        handle: "@acme".to_string(),
        id: "a41279e1-eaf5-5755-8860-1b6c8aebc4e3".to_string(),
        paused_reason: Some("Paused reason".to_string()),
        platform: serde_json::from_value::<Platform>(serde_json::Value::String("X".to_string()))
            .unwrap_or_default(),
        status: "CONNECTED".to_string(),
        token_expires_at: Some("example".to_string()),
    })
}
