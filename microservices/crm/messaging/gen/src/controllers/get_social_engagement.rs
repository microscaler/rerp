// User-owned controller for handler 'get_social_engagement'.

use crate::handlers::get_social_engagement::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetSocialEngagementController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "account_id": "a41279e1-eaf5-5755-8860-1b6c8aebc4e3",
    //       "engagements": 2100,
    //       "impressions": 128000,
    //       "platform": "X",
    //       "posts_30d": 41
    //     },
    //     {
    //       "account_id": "5bc54618-c69f-51c2-b8c5-adc463b9e422",
    //       "engagements": 3400,
    //       "impressions": 64000,
    //       "platform": "LINKEDIN",
    //       "posts_30d": 18
    //     },
    //     {
    //       "account_id": "d364f316-32c7-5b70-acc1-f13470095b2a",
    //       "engagements": 600,
    //       "impressions": 22000,
    //       "platform": "THREADS",
    //       "posts_30d": 30
    //     }
    //   ],
    //   "leads_captured_30d": 12
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "account_id": "a41279e1-eaf5-5755-8860-1b6c8aebc4e3",
      "engagements": 2100,
      "impressions": 128000,
      "platform": "X",
      "posts_30d": 41
    },
    {
      "account_id": "5bc54618-c69f-51c2-b8c5-adc463b9e422",
      "engagements": 3400,
      "impressions": 64000,
      "platform": "LINKEDIN",
      "posts_30d": 18
    },
    {
      "account_id": "d364f316-32c7-5b70-acc1-f13470095b2a",
      "engagements": 600,
      "impressions": 22000,
      "platform": "THREADS",
      "posts_30d": 30
    }
  ],
  "leads_captured_30d": 12
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_str::<serde_json::Value>(r#"{"account_id":"a41279e1-eaf5-5755-8860-1b6c8aebc4e3","engagements":2100,"impressions":128000,"platform":"X","posts_30d":41}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"account_id":"5bc54618-c69f-51c2-b8c5-adc463b9e422","engagements":3400,"impressions":64000,"platform":"LINKEDIN","posts_30d":18}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"account_id":"d364f316-32c7-5b70-acc1-f13470095b2a","engagements":600,"impressions":22000,"platform":"THREADS","posts_30d":30}"#).unwrap_or(serde_json::Value::Null)],leads_captured_30d: Some(12),
    })
}
