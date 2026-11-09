// User-owned controller for handler 'get_predictive_content'.

use crate::handlers::get_predictive_content::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::NameCount;
#[allow(unused_imports)]
use crate::handlers::types::VariantRow;

#[handler(GetPredictiveContentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "click_to_open_rate": 0.241,
    //   "clicks_by_variant": [
    //     {
    //       "count": 650,
    //       "name": "A"
    //     },
    //     {
    //       "count": 558,
    //       "name": "B"
    //     }
    //   ],
    //   "sent_by_variant": [
    //     {
    //       "count": 6342,
    //       "name": "A"
    //     },
    //     {
    //       "count": 6342,
    //       "name": "B"
    //     }
    //   ],
    //   "total_sent": 12684,
    //   "total_unique_clicks": 1208,
    //   "variants": [
    //     {
    //       "benchmark_click_to_open_rate": 0.21,
    //       "campaign_id": "7de39bcc-4bc6-551b-a967-7b1629b846be",
    //       "campaign_name": "October product update",
    //       "click_to_open_rate": 0.253,
    //       "sent": 4935,
    //       "sent_at": "2026-10-28T16:00:00Z",
    //       "unique_clicks": 512,
    //       "unique_opens": 2024,
    //       "variant_id": "2658de4b-a01f-55e5-bed6-e89621ad2615",
    //       "variant_name": "A · \"What's new\""
    //     },
    //     {
    //       "benchmark_click_to_open_rate": 0.21,
    //       "campaign_id": "7de39bcc-4bc6-551b-a967-7b1629b846be",
    //       "campaign_name": "October product update",
    //       "click_to_open_rate": 0.21,
    //       "sent": 4935,
    //       "sent_at": "2026-10-28T16:00:00Z",
    //       "unique_clicks": 423,
    //       "unique_opens": 2019,
    //       "variant_id": "07d19954-4b62-52e4-9524-44abfb643786",
    //       "variant_name": "B · \"3 things we shipped\""
    //     },
    //     {
    //       "benchmark_click_to_open_rate": 0.224,
    //       "campaign_id": "cd847d34-2090-5e40-afc2-06c94055cc85",
    //       "campaign_name": "Webinar invite · pricing strategy",
    //       "click_to_open_rate": 0.27,
    //       "sent": 1407,
    //       "sent_at": "2026-10-14T16:00:00Z",
    //       "unique_clicks": 138,
    //       "unique_opens": 512,
    //       "variant_id": "ef2a8fba-f07c-50df-ac44-f2500352490a",
    //       "variant_name": "A · benefit-led"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "click_to_open_rate": 0.241,
  "clicks_by_variant": [
    {
      "count": 650,
      "name": "A"
    },
    {
      "count": 558,
      "name": "B"
    }
  ],
  "sent_by_variant": [
    {
      "count": 6342,
      "name": "A"
    },
    {
      "count": 6342,
      "name": "B"
    }
  ],
  "total_sent": 12684,
  "total_unique_clicks": 1208,
  "variants": [
    {
      "benchmark_click_to_open_rate": 0.21,
      "campaign_id": "7de39bcc-4bc6-551b-a967-7b1629b846be",
      "campaign_name": "October product update",
      "click_to_open_rate": 0.253,
      "sent": 4935,
      "sent_at": "2026-10-28T16:00:00Z",
      "unique_clicks": 512,
      "unique_opens": 2024,
      "variant_id": "2658de4b-a01f-55e5-bed6-e89621ad2615",
      "variant_name": "A · \"What's new\""
    },
    {
      "benchmark_click_to_open_rate": 0.21,
      "campaign_id": "7de39bcc-4bc6-551b-a967-7b1629b846be",
      "campaign_name": "October product update",
      "click_to_open_rate": 0.21,
      "sent": 4935,
      "sent_at": "2026-10-28T16:00:00Z",
      "unique_clicks": 423,
      "unique_opens": 2019,
      "variant_id": "07d19954-4b62-52e4-9524-44abfb643786",
      "variant_name": "B · \"3 things we shipped\""
    },
    {
      "benchmark_click_to_open_rate": 0.224,
      "campaign_id": "cd847d34-2090-5e40-afc2-06c94055cc85",
      "campaign_name": "Webinar invite · pricing strategy",
      "click_to_open_rate": 0.27,
      "sent": 1407,
      "sent_at": "2026-10-14T16:00:00Z",
      "unique_clicks": 138,
      "unique_opens": 512,
      "variant_id": "ef2a8fba-f07c-50df-ac44-f2500352490a",
      "variant_name": "A · benefit-led"
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
        click_to_open_rate: Some(0.241),clicks_by_variant: Some(vec![serde_json::from_value::<NameCount>(serde_json::json!({"count":650,"name":"A"})).unwrap_or_default(), serde_json::from_value::<NameCount>(serde_json::json!({"count":558,"name":"B"})).unwrap_or_default()]),sent_by_variant: Some(vec![serde_json::from_value::<NameCount>(serde_json::json!({"count":6342,"name":"A"})).unwrap_or_default(), serde_json::from_value::<NameCount>(serde_json::json!({"count":6342,"name":"B"})).unwrap_or_default()]),total_sent: 12684,total_unique_clicks: 1208,variants: vec![serde_json::from_value::<VariantRow>(serde_json::json!({"benchmark_click_to_open_rate":0.21,"campaign_id":"7de39bcc-4bc6-551b-a967-7b1629b846be","campaign_name":"October product update","click_to_open_rate":0.253,"sent":4935,"sent_at":"2026-10-28T16:00:00Z","unique_clicks":512,"unique_opens":2024,"variant_id":"2658de4b-a01f-55e5-bed6-e89621ad2615","variant_name":"A · \"What's new\""})).unwrap_or_default(), serde_json::from_value::<VariantRow>(serde_json::json!({"benchmark_click_to_open_rate":0.21,"campaign_id":"7de39bcc-4bc6-551b-a967-7b1629b846be","campaign_name":"October product update","click_to_open_rate":0.21,"sent":4935,"sent_at":"2026-10-28T16:00:00Z","unique_clicks":423,"unique_opens":2019,"variant_id":"07d19954-4b62-52e4-9524-44abfb643786","variant_name":"B · \"3 things we shipped\""})).unwrap_or_default(), serde_json::from_value::<VariantRow>(serde_json::json!({"benchmark_click_to_open_rate":0.224,"campaign_id":"cd847d34-2090-5e40-afc2-06c94055cc85","campaign_name":"Webinar invite · pricing strategy","click_to_open_rate":0.27,"sent":1407,"sent_at":"2026-10-14T16:00:00Z","unique_clicks":138,"unique_opens":512,"variant_id":"ef2a8fba-f07c-50df-ac44-f2500352490a","variant_name":"A · benefit-led"})).unwrap_or_default()],
    })
}
