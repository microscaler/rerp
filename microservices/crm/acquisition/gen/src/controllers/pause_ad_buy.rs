// User-owned controller for handler 'pause_ad_buy'.

use crate::handlers::pause_ad_buy::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdBuySpec;

#[handler(PauseAdBuyController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "accounts": [
    //     {
    //       "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
    //       "clicks": 148,
    //       "external_adset_ids": [
    //         "adset 9920…"
    //       ],
    //       "external_campaign_id": "cmp 2381…",
    //       "frequency": 4.4,
    //       "impressions": 7057,
    //       "platform": "META",
    //       "reach": 1604,
    //       "review": "APPROVED",
    //       "spend": "812.00",
    //       "status": "LIVE",
    //       "unsupported": []
    //     },
    //     {
    //       "account_id": "1dcd203d-8734-5d40-b43b-9bbcdd9758b5",
    //       "clicks": 51,
    //       "external_adset_ids": [
    //         "Customer Match signal only"
    //       ],
    //       "external_campaign_id": "cmp 23873…",
    //       "frequency": 3.6,
    //       "impressions": 3643,
    //       "platform": "GOOGLE_ADS",
    //       "reach": 1012,
    //       "review": "APPROVED",
    //       "spend": "418.00",
    //       "status": "LIVE",
    //       "unsupported": [
    //         "frequency cap (display only)",
    //         "dayparting on YouTube"
    //       ]
    //     },
    //     {
    //       "account_id": "6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa",
    //       "clicks": 15,
    //       "external_adset_ids": [
    //         "cmp 2201…"
    //       ],
    //       "external_campaign_id": "cmpgrp 5089…",
    //       "frequency": 4.1,
    //       "impressions": 1492,
    //       "platform": "LINKEDIN",
    //       "reach": 364,
    //       "review": "PENDING",
    //       "spend": "230.00",
    //       "status": "LIVE",
    //       "unsupported": [
    //         "cost cap (uses max CPM)"
    //       ]
    //     }
    //   ],
    //   "created_at": "2026-10-27T16:00:00Z",
    //   "created_by": "alex",
    //   "id": "3e845a81-8fde-5036-87dc-3c182dc80f7c",
    //   "name": "Warm-up · Black Friday early access",
    //   "purpose": "WARM_UP",
    //   "spec": {
    //     "bid": {
    //       "amount": "1250.00",
    //       "metric": "CPM",
    //       "strategy": "LOWEST_COST"
    //     },
    //     "budget": {
    //       "amount": "1800.00",
    //       "currency": "EUR",
    //       "pacing": "EVEN",
    //       "period": "LIFETIME"
    //     },
    //     "creative_ids": [
    //       "30f4c733-b7f5-51d2-b900-0be84807ebb2",
    //       "8767be08-2adf-5232-a68d-eefde450d501"
    //     ],
    //     "frequency_cap": {
    //       "impressions": 4,
    //       "per_days": 7
    //     },
    //     "objective": "WARM_UP",
    //     "placements": [],
    //     "schedule": {
    //       "dayparts": [
    //         {
    //           "days": [
    //             "MON",
    //             "TUE",
    //             "WED"
    //           ],
    //           "from_hour": 7,
    //           "to_hour": 22
    //         }
    //       ],
    //       "end_at": "2026-11-18T16:00:00Z",
    //       "start_at": "2026-11-08T16:00:00Z",
    //       "timezone": "contact"
    //     },
    //     "targeting": {
    //       "audience_id": "b6d841b8-e819-50bb-ba55-f9f0c0e679bf",
    //       "devices": [
    //         "DESKTOP",
    //         "MOBILE",
    //         "TABLET"
    //       ],
    //       "exclude_audience_ids": [
    //         "e1668eef-08fd-5afb-bef2-1a2ac7dd3ba0"
    //       ],
    //       "geo": [
    //         "DE",
    //         "AT",
    //         "CH"
    //       ],
    //       "lookalike": false
    //     },
    //     "utm": {
    //       "utm_campaign": "bf-early-access",
    //       "utm_content": "{creative.id}",
    //       "utm_medium": "cpc",
    //       "utm_source": "{platform}"
    //     }
    //   },
    //   "status": "LIVE",
    //   "totals": {
    //     "attributed_leads": 9,
    //     "attributed_revenue": "1200.00",
    //     "clicks": 214,
    //     "reach": 2980,
    //     "spend": "1460.00"
    //   },
    //   "updated_at": "2026-11-05T15:00:00Z",
    //   "warm_up_for_campaign_id": "cf353d4a-972b-5b2f-8d04-b74803de4c99"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "accounts": [
    {
      "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
      "clicks": 148,
      "external_adset_ids": [
        "adset 9920…"
      ],
      "external_campaign_id": "cmp 2381…",
      "frequency": 4.4,
      "impressions": 7057,
      "platform": "META",
      "reach": 1604,
      "review": "APPROVED",
      "spend": "812.00",
      "status": "LIVE",
      "unsupported": []
    },
    {
      "account_id": "1dcd203d-8734-5d40-b43b-9bbcdd9758b5",
      "clicks": 51,
      "external_adset_ids": [
        "Customer Match signal only"
      ],
      "external_campaign_id": "cmp 23873…",
      "frequency": 3.6,
      "impressions": 3643,
      "platform": "GOOGLE_ADS",
      "reach": 1012,
      "review": "APPROVED",
      "spend": "418.00",
      "status": "LIVE",
      "unsupported": [
        "frequency cap (display only)",
        "dayparting on YouTube"
      ]
    },
    {
      "account_id": "6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa",
      "clicks": 15,
      "external_adset_ids": [
        "cmp 2201…"
      ],
      "external_campaign_id": "cmpgrp 5089…",
      "frequency": 4.1,
      "impressions": 1492,
      "platform": "LINKEDIN",
      "reach": 364,
      "review": "PENDING",
      "spend": "230.00",
      "status": "LIVE",
      "unsupported": [
        "cost cap (uses max CPM)"
      ]
    }
  ],
  "created_at": "2026-10-27T16:00:00Z",
  "created_by": "alex",
  "id": "3e845a81-8fde-5036-87dc-3c182dc80f7c",
  "name": "Warm-up · Black Friday early access",
  "purpose": "WARM_UP",
  "spec": {
    "bid": {
      "amount": "1250.00",
      "metric": "CPM",
      "strategy": "LOWEST_COST"
    },
    "budget": {
      "amount": "1800.00",
      "currency": "EUR",
      "pacing": "EVEN",
      "period": "LIFETIME"
    },
    "creative_ids": [
      "30f4c733-b7f5-51d2-b900-0be84807ebb2",
      "8767be08-2adf-5232-a68d-eefde450d501"
    ],
    "frequency_cap": {
      "impressions": 4,
      "per_days": 7
    },
    "objective": "WARM_UP",
    "placements": [],
    "schedule": {
      "dayparts": [
        {
          "days": [
            "MON",
            "TUE",
            "WED"
          ],
          "from_hour": 7,
          "to_hour": 22
        }
      ],
      "end_at": "2026-11-18T16:00:00Z",
      "start_at": "2026-11-08T16:00:00Z",
      "timezone": "contact"
    },
    "targeting": {
      "audience_id": "b6d841b8-e819-50bb-ba55-f9f0c0e679bf",
      "devices": [
        "DESKTOP",
        "MOBILE",
        "TABLET"
      ],
      "exclude_audience_ids": [
        "e1668eef-08fd-5afb-bef2-1a2ac7dd3ba0"
      ],
      "geo": [
        "DE",
        "AT",
        "CH"
      ],
      "lookalike": false
    },
    "utm": {
      "utm_campaign": "bf-early-access",
      "utm_content": "{creative.id}",
      "utm_medium": "cpc",
      "utm_source": "{platform}"
    }
  },
  "status": "LIVE",
  "totals": {
    "attributed_leads": 9,
    "attributed_revenue": "1200.00",
    "clicks": 214,
    "reach": 2980,
    "spend": "1460.00"
  },
  "updated_at": "2026-11-05T15:00:00Z",
  "warm_up_for_campaign_id": "cf353d4a-972b-5b2f-8d04-b74803de4c99"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        accounts: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{"account_id":"ded95315-6c77-51a0-b8ea-66b76fada21a","clicks":148,"external_adset_ids":["adset 9920…"],"external_campaign_id":"cmp 2381…","frequency":4.4,"impressions":7057,"platform":"META","reach":1604,"review":"APPROVED","spend":"812.00","status":"LIVE","unsupported":[]}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"account_id":"1dcd203d-8734-5d40-b43b-9bbcdd9758b5","clicks":51,"external_adset_ids":["Customer Match signal only"],"external_campaign_id":"cmp 23873…","frequency":3.6,"impressions":3643,"platform":"GOOGLE_ADS","reach":1012,"review":"APPROVED","spend":"418.00","status":"LIVE","unsupported":["frequency cap (display only)","dayparting on YouTube"]}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"account_id":"6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa","clicks":15,"external_adset_ids":["cmp 2201…"],"external_campaign_id":"cmpgrp 5089…","frequency":4.1,"impressions":1492,"platform":"LINKEDIN","reach":364,"review":"PENDING","spend":"230.00","status":"LIVE","unsupported":["cost cap (uses max CPM)"]}"#).unwrap_or(serde_json::Value::Null)]),created_at: Some("2026-10-27T16:00:00Z".to_string()),created_by: Some("alex".to_string()),id: "3e845a81-8fde-5036-87dc-3c182dc80f7c".to_string(),name: "Warm-up · Black Friday early access".to_string(),purpose: Some("WARM_UP".to_string()),spec: serde_json::from_value::<AdBuySpec>(serde_json::json!({"bid":{"amount":"1250.00","metric":"CPM","strategy":"LOWEST_COST"},"budget":{"amount":"1800.00","currency":"EUR","pacing":"EVEN","period":"LIFETIME"},"creative_ids":["30f4c733-b7f5-51d2-b900-0be84807ebb2","8767be08-2adf-5232-a68d-eefde450d501"],"frequency_cap":{"impressions":4,"per_days":7},"objective":"WARM_UP","placements":[],"schedule":{"dayparts":[{"days":["MON","TUE","WED"],"from_hour":7,"to_hour":22}],"end_at":"2026-11-18T16:00:00Z","start_at":"2026-11-08T16:00:00Z","timezone":"contact"},"targeting":{"audience_id":"b6d841b8-e819-50bb-ba55-f9f0c0e679bf","devices":["DESKTOP","MOBILE","TABLET"],"exclude_audience_ids":["e1668eef-08fd-5afb-bef2-1a2ac7dd3ba0"],"geo":["DE","AT","CH"],"lookalike":false},"utm":{"utm_campaign":"bf-early-access","utm_content":"{creative.id}","utm_medium":"cpc","utm_source":"{platform}"}})).unwrap_or_default(),status: "LIVE".to_string(),totals: Some(serde_json::from_str::<serde_json::Value>(r#"{"attributed_leads":9,"attributed_revenue":"1200.00","clicks":214,"reach":2980,"spend":"1460.00"}"#).unwrap_or(serde_json::Value::Null)),updated_at: Some("2026-11-05T15:00:00Z".to_string()),warm_up_for_campaign_id: Some("cf353d4a-972b-5b2f-8d04-b74803de4c99".to_string()),
    })
}
