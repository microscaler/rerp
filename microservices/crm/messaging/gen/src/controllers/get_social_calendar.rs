// User-owned controller for handler 'get_social_calendar'.

use crate::handlers::get_social_calendar::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetSocialCalendarController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "days": [
    //     {
    //       "date": "2026-11-25",
    //       "posts": [
    //         {
    //           "body": "Body",
    //           "created_at": "2026-11-05T16:00:00Z",
    //           "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
    //           "status": "DRAFT",
    //           "targets": [
    //             {
    //               "account_id": "b06c6126-3814-5778-9fd5-6fd9f1ad37bf"
    //             }
    //           ]
    //         }
    //       ]
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "days": [
    {
      "date": "2026-11-25",
      "posts": [
        {
          "body": "Body",
          "created_at": "2026-11-05T16:00:00Z",
          "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
          "status": "DRAFT",
          "targets": [
            {
              "account_id": "b06c6126-3814-5778-9fd5-6fd9f1ad37bf"
            }
          ]
        }
      ]
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
        days: vec![serde_json::from_str::<serde_json::Value>(r#"{"date":"2026-11-25","posts":[{"body":"Body","created_at":"2026-11-05T16:00:00Z","id":"3eb009b7-a99d-5a4c-956a-99e11d8dbb76","status":"DRAFT","targets":[{"account_id":"b06c6126-3814-5778-9fd5-6fd9f1ad37bf"}]}]}"#).unwrap_or(serde_json::Value::Null)],
    })
}
