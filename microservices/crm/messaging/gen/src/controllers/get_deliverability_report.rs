// User-owned controller for handler 'get_deliverability_report'.

use crate::handlers::get_deliverability_report::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetDeliverabilityReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "by_provider": [
    //     {
    //       "bounce_rate": 0.001,
    //       "open_rate": 0.4,
    //       "provider": "Gmail / Workspace",
    //       "share": 0.54
    //     },
    //     {
    //       "bounce_rate": 0.003,
    //       "open_rate": 0.35,
    //       "provider": "Microsoft 365 / Outlook",
    //       "share": 0.31
    //     },
    //     {
    //       "bounce_rate": 0.004,
    //       "open_rate": 0.31,
    //       "provider": "Yahoo / AOL",
    //       "share": 0.05
    //     }
    //   ],
    //   "complaint_rate": 0.0002,
    //   "delivered_rate": 0.987,
    //   "domain_reputation": "GOOD",
    //   "hard_bounce_rate": 0.002,
    //   "soft_bounce_rate": 0.007,
    //   "suppressed_30d": 122
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "by_provider": [
    {
      "bounce_rate": 0.001,
      "open_rate": 0.4,
      "provider": "Gmail / Workspace",
      "share": 0.54
    },
    {
      "bounce_rate": 0.003,
      "open_rate": 0.35,
      "provider": "Microsoft 365 / Outlook",
      "share": 0.31
    },
    {
      "bounce_rate": 0.004,
      "open_rate": 0.31,
      "provider": "Yahoo / AOL",
      "share": 0.05
    }
  ],
  "complaint_rate": 0.0002,
  "delivered_rate": 0.987,
  "domain_reputation": "GOOD",
  "hard_bounce_rate": 0.002,
  "soft_bounce_rate": 0.007,
  "suppressed_30d": 122
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        by_domain: Some(vec![]),by_provider: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{"bounce_rate":0.001,"open_rate":0.4,"provider":"Gmail / Workspace","share":0.54}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"bounce_rate":0.003,"open_rate":0.35,"provider":"Microsoft 365 / Outlook","share":0.31}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"bounce_rate":0.004,"open_rate":0.31,"provider":"Yahoo / AOL","share":0.05}"#).unwrap_or(serde_json::Value::Null)]),complaint_rate: Some(0.0002),delivered_rate: Some(0.987),domain_reputation: Some("GOOD".to_string()),from: Some("example".to_string()),hard_bounce_rate: Some(0.002),soft_bounce_rate: Some(0.007),suppressed_30d: Some(122),to: Some("example".to_string()),
    })
}
