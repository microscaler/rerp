// User-owned controller for handler 'get_account_engagement'.

use crate::handlers::get_account_engagement::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetAccountEngagementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "account_id": "b06c6126-3814-5778-9fd5-6fd9f1ad37bf",
    //   "points": [
    //     {
    //       "date": "2026-11-25"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "account_id": "b06c6126-3814-5778-9fd5-6fd9f1ad37bf",
  "points": [
    {
      "date": "2026-11-25"
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

    HttpJson::ok(Response {
        account_id: "b06c6126-3814-5778-9fd5-6fd9f1ad37bf".to_string(),
        points: vec![
            serde_json::from_str::<serde_json::Value>(r#"{"date":"2026-11-25"}"#)
                .unwrap_or(serde_json::Value::Null),
        ],
    })
}
