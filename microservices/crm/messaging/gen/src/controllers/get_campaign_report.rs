// User-owned controller for handler 'get_campaign_report'.

use crate::handlers::get_campaign_report::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::CampaignVariant;

#[handler(GetCampaignReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "audience": {},
    //   "bounced": 135,
    //   "campaign_id": "7de39bcc-4bc6-551b-a967-7b1629b846be",
    //   "click_rate": 0.052,
    //   "click_to_open_rate": 0.127,
    //   "clicked": 513,
    //   "complained": 3,
    //   "delivered": 9732,
    //   "links": [
    //     {
    //       "clicks": 412,
    //       "unique_clicks": 388,
    //       "url": "https://acme.example/changelog/october"
    //     },
    //     {
    //       "clicks": 161,
    //       "unique_clicks": 152,
    //       "url": "https://acme.example/docs/imports"
    //     },
    //     {
    //       "clicks": 118,
    //       "unique_clicks": 109,
    //       "url": "https://acme.example/webinars/pricing"
    //     }
    //   ],
    //   "open_rate": 0.41,
    //   "opened": 4047,
    //   "opens_by_hour": [
    //     {
    //       "clicks": 50,
    //       "hour": "2026-10-27T16:00:00Z",
    //       "opens": 280
    //     },
    //     {
    //       "clicks": 138,
    //       "hour": "2026-10-27T17:00:00Z",
    //       "opens": 770
    //     },
    //     {
    //       "clicks": 238,
    //       "hour": "2026-10-27T18:00:00Z",
    //       "opens": 1330
    //     }
    //   ],
    //   "revenue": "6240.00",
    //   "sent": 9870,
    //   "social_shares": [
    //     {}
    //   ],
    //   "total_clicks": 777,
    //   "total_opens": 5447,
    //   "unsubscribed": 12,
    //   "verified_opens": 4045
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "audience": {},
  "bounced": 135,
  "campaign_id": "7de39bcc-4bc6-551b-a967-7b1629b846be",
  "click_rate": 0.052,
  "click_to_open_rate": 0.127,
  "clicked": 513,
  "complained": 3,
  "delivered": 9732,
  "links": [
    {
      "clicks": 412,
      "unique_clicks": 388,
      "url": "https://acme.example/changelog/october"
    },
    {
      "clicks": 161,
      "unique_clicks": 152,
      "url": "https://acme.example/docs/imports"
    },
    {
      "clicks": 118,
      "unique_clicks": 109,
      "url": "https://acme.example/webinars/pricing"
    }
  ],
  "open_rate": 0.41,
  "opened": 4047,
  "opens_by_hour": [
    {
      "clicks": 50,
      "hour": "2026-10-27T16:00:00Z",
      "opens": 280
    },
    {
      "clicks": 138,
      "hour": "2026-10-27T17:00:00Z",
      "opens": 770
    },
    {
      "clicks": 238,
      "hour": "2026-10-27T18:00:00Z",
      "opens": 1330
    }
  ],
  "revenue": "6240.00",
  "sent": 9870,
  "social_shares": [
    {}
  ],
  "total_clicks": 777,
  "total_opens": 5447,
  "unsubscribed": 12,
  "verified_opens": 4045
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        anonymous_open_rate: Some(1.5),audience: Some(serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)),bounced: 135,campaign_id: "7de39bcc-4bc6-551b-a967-7b1629b846be".to_string(),click_rate: Some(0.052),click_to_open_rate: Some(0.127),clicked: 513,complained: 3,delivered: 9732,email_clients: Some(vec![]),forwarded: Some(42),from: Some("example".to_string()),links: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{"clicks":412,"unique_clicks":388,"url":"https://acme.example/changelog/october"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"clicks":161,"unique_clicks":152,"url":"https://acme.example/docs/imports"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"clicks":118,"unique_clicks":109,"url":"https://acme.example/webinars/pricing"}"#).unwrap_or(serde_json::Value::Null)]),open_rate: Some(0.41),opened: 4047,opens_by_date: Some(vec![]),opens_by_hour: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{"clicks":50,"hour":"2026-10-27T16:00:00Z","opens":280}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"clicks":138,"hour":"2026-10-27T17:00:00Z","opens":770}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"clicks":238,"hour":"2026-10-27T18:00:00Z","opens":1330}"#).unwrap_or(serde_json::Value::Null)]),opens_by_location: Some(vec![]),replied: Some(42),revenue: Some("6240.00".to_string()),sent: 9870,sent_at: Some("example".to_string()),social_shares: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)]),subject: Some("example".to_string()),total_clicks: Some(777),total_opens: Some(5447),unsubscribed: 12,unverified_opens: Some(42),updated: Some(42),variants: Some(vec![]),verified_opens: Some(4045),
    })
}
