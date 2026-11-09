// User-owned controller for handler 'get_post_engagement'.

use crate::handlers::get_post_engagement::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Engagement;

#[handler(GetPostEngagementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "account_id": "b06c6126-3814-5778-9fd5-6fd9f1ad37bf",
    //       "fetched_at": "2026-11-05T16:00:00Z",
    //       "platform": "X"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "account_id": "b06c6126-3814-5778-9fd5-6fd9f1ad37bf",
      "fetched_at": "2026-11-05T16:00:00Z",
      "platform": "X"
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

    HttpJson::ok(Response(vec![serde_json::from_value::<Engagement>(serde_json::json!({"account_id":"b06c6126-3814-5778-9fd5-6fd9f1ad37bf","fetched_at":"2026-11-05T16:00:00Z","platform":"X"})).unwrap_or_default()]))
}
