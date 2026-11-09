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
    //   "by_location": [
    //     {
    //       "clicks": 188,
    //       "name": "United Kingdom · London",
    //       "opens": 1412,
    //       "share_pct": 35
    //     },
    //     {
    //       "clicks": 96,
    //       "name": "Germany · Berlin",
    //       "opens": 804,
    //       "share_pct": 20
    //     },
    //     {
    //       "clicks": 71,
    //       "name": "Bulgaria · Sofia",
    //       "opens": 611,
    //       "share_pct": 15
    //     }
    //   ],
    //   "campaign_id": "7de39bcc-4bc6-551b-a967-7b1629b846be",
    //   "click_rate": 0.052,
    //   "click_to_open_rate": 0.127,
    //   "clicked": 513,
    //   "clients": [
    //     {
    //       "client": "Gmail",
    //       "share_pct": 46,
    //       "verified_pct": 98
    //     },
    //     {
    //       "client": "Apple Mail",
    //       "share_pct": 24,
    //       "verified_pct": 28
    //     },
    //     {
    //       "client": "Outlook",
    //       "share_pct": 18,
    //       "verified_pct": 97
    //     }
    //   ],
    //   "complained": 3,
    //   "deals_won": 3,
    //   "delivered": 9732,
    //   "delivery": [
    //     {
    //       "count": 9732,
    //       "label": "Delivered"
    //     },
    //     {
    //       "count": 97,
    //       "label": "Soft bounce"
    //     },
    //     {
    //       "count": 38,
    //       "label": "Hard bounce"
    //     }
    //   ],
    //   "forwards": 14,
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
    //   "open_rate_vs_list_pts": 2.8,
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
    //   "prefetch_opens": 1402,
    //   "profile_updates": 27,
    //   "replies": 9,
    //   "revenue": "6240.00",
    //   "sent": 9870,
    //   "social_shares": [
    //     {}
    //   ],
    //   "social_shares_by": [
    //     {
    //       "count": 5,
    //       "name": "LinkedIn"
    //     },
    //     {
    //       "count": 1,
    //       "name": "X"
    //     }
    //   ],
    //   "suppressions_added": 41,
    //   "total_clicks": 777,
    //   "total_opens": 5447,
    //   "unsubscribed": 12,
    //   "verified_opens": 4045
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "audience": {},
  "bounced": 135,
  "by_location": [
    {
      "clicks": 188,
      "name": "United Kingdom · London",
      "opens": 1412,
      "share_pct": 35
    },
    {
      "clicks": 96,
      "name": "Germany · Berlin",
      "opens": 804,
      "share_pct": 20
    },
    {
      "clicks": 71,
      "name": "Bulgaria · Sofia",
      "opens": 611,
      "share_pct": 15
    }
  ],
  "campaign_id": "7de39bcc-4bc6-551b-a967-7b1629b846be",
  "click_rate": 0.052,
  "click_to_open_rate": 0.127,
  "clicked": 513,
  "clients": [
    {
      "client": "Gmail",
      "share_pct": 46,
      "verified_pct": 98
    },
    {
      "client": "Apple Mail",
      "share_pct": 24,
      "verified_pct": 28
    },
    {
      "client": "Outlook",
      "share_pct": 18,
      "verified_pct": 97
    }
  ],
  "complained": 3,
  "deals_won": 3,
  "delivered": 9732,
  "delivery": [
    {
      "count": 9732,
      "label": "Delivered"
    },
    {
      "count": 97,
      "label": "Soft bounce"
    },
    {
      "count": 38,
      "label": "Hard bounce"
    }
  ],
  "forwards": 14,
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
  "open_rate_vs_list_pts": 2.8,
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
  "prefetch_opens": 1402,
  "profile_updates": 27,
  "replies": 9,
  "revenue": "6240.00",
  "sent": 9870,
  "social_shares": [
    {}
  ],
  "social_shares_by": [
    {
      "count": 5,
      "name": "LinkedIn"
    },
    {
      "count": 1,
      "name": "X"
    }
  ],
  "suppressions_added": 41,
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
        anonymous_open_rate: Some(1.5),audience: Some(serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)),bounced: 135,by_location: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{"clicks":188,"name":"United Kingdom · London","opens":1412,"share_pct":35}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"clicks":96,"name":"Germany · Berlin","opens":804,"share_pct":20}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"clicks":71,"name":"Bulgaria · Sofia","opens":611,"share_pct":15}"#).unwrap_or(serde_json::Value::Null)]),campaign_id: "7de39bcc-4bc6-551b-a967-7b1629b846be".to_string(),click_rate: Some(0.052),click_to_open_rate: Some(0.127),clicked: 513,clients: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{"client":"Gmail","share_pct":46,"verified_pct":98}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"client":"Apple Mail","share_pct":24,"verified_pct":28}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"client":"Outlook","share_pct":18,"verified_pct":97}"#).unwrap_or(serde_json::Value::Null)]),complained: 3,deals_won: Some(3),delivered: 9732,delivery: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{"count":9732,"label":"Delivered"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"count":97,"label":"Soft bounce"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"count":38,"label":"Hard bounce"}"#).unwrap_or(serde_json::Value::Null)]),email_clients: Some(vec![]),forwarded: Some(42),forwards: Some(14),from: Some("example".to_string()),links: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{"clicks":412,"unique_clicks":388,"url":"https://acme.example/changelog/october"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"clicks":161,"unique_clicks":152,"url":"https://acme.example/docs/imports"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"clicks":118,"unique_clicks":109,"url":"https://acme.example/webinars/pricing"}"#).unwrap_or(serde_json::Value::Null)]),open_rate: Some(0.41),open_rate_vs_list_pts: Some(2.8),opened: 4047,opens_by_date: Some(vec![]),opens_by_hour: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{"clicks":50,"hour":"2026-10-27T16:00:00Z","opens":280}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"clicks":138,"hour":"2026-10-27T17:00:00Z","opens":770}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"clicks":238,"hour":"2026-10-27T18:00:00Z","opens":1330}"#).unwrap_or(serde_json::Value::Null)]),opens_by_location: Some(vec![]),prefetch_opens: Some(1402),profile_updates: Some(27),replied: Some(42),replies: Some(9),revenue: Some("6240.00".to_string()),sent: 9870,sent_at: Some("example".to_string()),social_shares: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)]),social_shares_by: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{"count":5,"name":"LinkedIn"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"count":1,"name":"X"}"#).unwrap_or(serde_json::Value::Null)]),subject: Some("example".to_string()),suppressions_added: Some(41),total_clicks: Some(777),total_opens: Some(5447),unsubscribed: 12,unverified_opens: Some(42),updated: Some(42),variants: Some(vec![]),verified_opens: Some(4045),
    })
}
