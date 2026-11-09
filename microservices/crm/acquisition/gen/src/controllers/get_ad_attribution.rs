// User-owned controller for handler 'get_ad_attribution'.

use crate::handlers::get_ad_attribution::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetAdAttributionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "from": "2026-10-06",
    //   "model": "last_touch",
    //   "rows": [
    //     {
    //       "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
    //       "cac": "1460.00",
    //       "clicks": 214,
    //       "deals": 3,
    //       "impressions": 71000,
    //       "key": "262ac3c8-b3f1-5aff-a843-87613a514a9e",
    //       "label": "BF early access · warm-up",
    //       "leads": 9,
    //       "ltv_90d": 760,
    //       "platform": "Meta",
    //       "platform_reported_conversions": 11,
    //       "platform_reported_revenue": "9800.00",
    //       "recommendation": "HOLD",
    //       "recommendation_change_pct": null,
    //       "recommendation_note": "lift after send",
    //       "revenue": "2400.00",
    //       "revenue_by_position": {
    //         "assists": 4800,
    //         "first": 1200,
    //         "last": 2400
    //       },
    //       "roas": 1.6,
    //       "spend": "1460.00",
    //       "won": 1
    //     },
    //     {
    //       "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
    //       "cac": "124.00",
    //       "clicks": 402,
    //       "deals": 8,
    //       "impressions": 38000,
    //       "key": "f3cd6604-1b26-578c-b830-9a23140db612",
    //       "label": "Retarget · pricing visitors",
    //       "leads": 21,
    //       "ltv_90d": 1240,
    //       "platform": "Meta + Google",
    //       "platform_reported_conversions": 14,
    //       "platform_reported_revenue": "12100.00",
    //       "recommendation": "SCALE",
    //       "recommendation_change_pct": "Recommendation change pct",
    //       "recommendation_note": "+20%",
    //       "revenue": "3900.00",
    //       "revenue_by_position": {
    //         "assists": 2100,
    //         "first": 7400,
    //         "last": 3900
    //       },
    //       "roas": 6.3,
    //       "spend": "620.00",
    //       "won": 5
    //     },
    //     {
    //       "account_id": "6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa",
    //       "cac": "300.00",
    //       "clicks": 118,
    //       "deals": 5,
    //       "impressions": 22000,
    //       "key": "88429ed5-a8b0-5109-affc-e41b18874962",
    //       "label": "Webinar invite · warm-up",
    //       "leads": 14,
    //       "ltv_90d": 2010,
    //       "platform": "LinkedIn",
    //       "platform_reported_conversions": 6,
    //       "platform_reported_revenue": "4200.00",
    //       "recommendation": "HOLD",
    //       "recommendation_change_pct": null,
    //       "recommendation_note": "",
    //       "revenue": "1400.00",
    //       "revenue_by_position": {
    //         "assists": 3100,
    //         "first": 2600,
    //         "last": 1400
    //       },
    //       "roas": 2.3,
    //       "spend": "600.00",
    //       "won": 2
    //     }
    //   ],
    //   "to": "2026-11-05",
    //   "totals": {
    //     "accounts": 5,
    //     "cac": 114,
    //     "deals": 29,
    //     "leads": 95,
    //     "platform_reported_revenue": "38900.00",
    //     "revenue": "21400.00",
    //     "roas": 5.2,
    //     "spend": "4120.00",
    //     "unattributed_revenue": "14200.00",
    //     "won": 36
    //   },
    //   "warm_up_lift": {
    //     "campaign_name": "Black Friday early access",
    //     "cost": 600,
    //     "cost_per_contact": 0.33,
    //     "exposed": {
    //       "click_rate": 0.079,
    //       "contacts": 1804,
    //       "open_rate": 0.412,
    //       "reply_rate": 0.014,
    //       "unsub_rate": 0.003
    //     },
    //     "extra_clicks_per_100": 11,
    //     "holdout": {
    //       "click_rate": 0.061,
    //       "contacts": 318,
    //       "open_rate": 0.36,
    //       "reply_rate": 0.011,
    //       "unsub_rate": 0.003
    //     },
    //     "measured_after": "2026-11-18T16:00:00Z",
    //     "note": "Measured after the send; below are the October webinar warm-up's numbers.",
    //     "relative_lift": {
    //       "click_rate": 0.3,
    //       "open_rate": 0.14,
    //       "reply_rate": 0.27,
    //       "unsub_rate": 0
    //     }
    //   },
    //   "window_days": 30
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "from": "2026-10-06",
  "model": "last_touch",
  "rows": [
    {
      "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
      "cac": "1460.00",
      "clicks": 214,
      "deals": 3,
      "impressions": 71000,
      "key": "262ac3c8-b3f1-5aff-a843-87613a514a9e",
      "label": "BF early access · warm-up",
      "leads": 9,
      "ltv_90d": 760,
      "platform": "Meta",
      "platform_reported_conversions": 11,
      "platform_reported_revenue": "9800.00",
      "recommendation": "HOLD",
      "recommendation_change_pct": null,
      "recommendation_note": "lift after send",
      "revenue": "2400.00",
      "revenue_by_position": {
        "assists": 4800,
        "first": 1200,
        "last": 2400
      },
      "roas": 1.6,
      "spend": "1460.00",
      "won": 1
    },
    {
      "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
      "cac": "124.00",
      "clicks": 402,
      "deals": 8,
      "impressions": 38000,
      "key": "f3cd6604-1b26-578c-b830-9a23140db612",
      "label": "Retarget · pricing visitors",
      "leads": 21,
      "ltv_90d": 1240,
      "platform": "Meta + Google",
      "platform_reported_conversions": 14,
      "platform_reported_revenue": "12100.00",
      "recommendation": "SCALE",
      "recommendation_change_pct": "Recommendation change pct",
      "recommendation_note": "+20%",
      "revenue": "3900.00",
      "revenue_by_position": {
        "assists": 2100,
        "first": 7400,
        "last": 3900
      },
      "roas": 6.3,
      "spend": "620.00",
      "won": 5
    },
    {
      "account_id": "6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa",
      "cac": "300.00",
      "clicks": 118,
      "deals": 5,
      "impressions": 22000,
      "key": "88429ed5-a8b0-5109-affc-e41b18874962",
      "label": "Webinar invite · warm-up",
      "leads": 14,
      "ltv_90d": 2010,
      "platform": "LinkedIn",
      "platform_reported_conversions": 6,
      "platform_reported_revenue": "4200.00",
      "recommendation": "HOLD",
      "recommendation_change_pct": null,
      "recommendation_note": "",
      "revenue": "1400.00",
      "revenue_by_position": {
        "assists": 3100,
        "first": 2600,
        "last": 1400
      },
      "roas": 2.3,
      "spend": "600.00",
      "won": 2
    }
  ],
  "to": "2026-11-05",
  "totals": {
    "accounts": 5,
    "cac": 114,
    "deals": 29,
    "leads": 95,
    "platform_reported_revenue": "38900.00",
    "revenue": "21400.00",
    "roas": 5.2,
    "spend": "4120.00",
    "unattributed_revenue": "14200.00",
    "won": 36
  },
  "warm_up_lift": {
    "campaign_name": "Black Friday early access",
    "cost": 600,
    "cost_per_contact": 0.33,
    "exposed": {
      "click_rate": 0.079,
      "contacts": 1804,
      "open_rate": 0.412,
      "reply_rate": 0.014,
      "unsub_rate": 0.003
    },
    "extra_clicks_per_100": 11,
    "holdout": {
      "click_rate": 0.061,
      "contacts": 318,
      "open_rate": 0.36,
      "reply_rate": 0.011,
      "unsub_rate": 0.003
    },
    "measured_after": "2026-11-18T16:00:00Z",
    "note": "Measured after the send; below are the October webinar warm-up's numbers.",
    "relative_lift": {
      "click_rate": 0.3,
      "open_rate": 0.14,
      "reply_rate": 0.27,
      "unsub_rate": 0
    }
  },
  "window_days": 30
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        from: Some("2026-10-06".to_string()),model: Some("last_touch".to_string()),rows: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{"account_id":"ded95315-6c77-51a0-b8ea-66b76fada21a","cac":"1460.00","clicks":214,"deals":3,"impressions":71000,"key":"262ac3c8-b3f1-5aff-a843-87613a514a9e","label":"BF early access · warm-up","leads":9,"ltv_90d":760,"platform":"Meta","platform_reported_conversions":11,"platform_reported_revenue":"9800.00","recommendation":"HOLD","recommendation_change_pct":null,"recommendation_note":"lift after send","revenue":"2400.00","revenue_by_position":{"assists":4800,"first":1200,"last":2400},"roas":1.6,"spend":"1460.00","won":1}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"account_id":"ded95315-6c77-51a0-b8ea-66b76fada21a","cac":"124.00","clicks":402,"deals":8,"impressions":38000,"key":"f3cd6604-1b26-578c-b830-9a23140db612","label":"Retarget · pricing visitors","leads":21,"ltv_90d":1240,"platform":"Meta + Google","platform_reported_conversions":14,"platform_reported_revenue":"12100.00","recommendation":"SCALE","recommendation_change_pct":"Recommendation change pct","recommendation_note":"+20%","revenue":"3900.00","revenue_by_position":{"assists":2100,"first":7400,"last":3900},"roas":6.3,"spend":"620.00","won":5}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"account_id":"6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa","cac":"300.00","clicks":118,"deals":5,"impressions":22000,"key":"88429ed5-a8b0-5109-affc-e41b18874962","label":"Webinar invite · warm-up","leads":14,"ltv_90d":2010,"platform":"LinkedIn","platform_reported_conversions":6,"platform_reported_revenue":"4200.00","recommendation":"HOLD","recommendation_change_pct":null,"recommendation_note":"","revenue":"1400.00","revenue_by_position":{"assists":3100,"first":2600,"last":1400},"roas":2.3,"spend":"600.00","won":2}"#).unwrap_or(serde_json::Value::Null)]),to: Some("2026-11-05".to_string()),totals: Some(serde_json::from_str::<serde_json::Value>(r#"{"accounts":5,"cac":114,"deals":29,"leads":95,"platform_reported_revenue":"38900.00","revenue":"21400.00","roas":5.2,"spend":"4120.00","unattributed_revenue":"14200.00","won":36}"#).unwrap_or(serde_json::Value::Null)),warm_up_lift: Some(serde_json::from_str::<serde_json::Value>(r#"{"campaign_name":"Black Friday early access","cost":600,"cost_per_contact":0.33,"exposed":{"click_rate":0.079,"contacts":1804,"open_rate":0.412,"reply_rate":0.014,"unsub_rate":0.003},"extra_clicks_per_100":11,"holdout":{"click_rate":0.061,"contacts":318,"open_rate":0.36,"reply_rate":0.011,"unsub_rate":0.003},"measured_after":"2026-11-18T16:00:00Z","note":"Measured after the send; below are the October webinar warm-up's numbers.","relative_lift":{"click_rate":0.3,"open_rate":0.14,"reply_rate":0.27,"unsub_rate":0}}"#).unwrap_or(serde_json::Value::Null)),window_days: Some(30),
    })
}
