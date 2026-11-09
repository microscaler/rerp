// User-owned controller for handler 'list_social_accounts'.

use crate::handlers::list_social_accounts::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::SocialAccount;

#[handler(ListSocialAccountsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "created_at": "2026-11-05T16:00:00Z",
    //       "handle": "Handle",
    //       "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
    //       "platform": "X",
    //       "status": "CONNECTING"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "created_at": "2026-11-05T16:00:00Z",
      "handle": "Handle",
      "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
      "platform": "X",
      "status": "CONNECTING"
    }
  ]
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response(vec![serde_json::from_value::<SocialAccount>(serde_json::json!({"created_at":"2026-11-05T16:00:00Z","handle":"Handle","id":"3eb009b7-a99d-5a4c-956a-99e11d8dbb76","platform":"X","status":"CONNECTING"})).unwrap_or_default()]))
}
