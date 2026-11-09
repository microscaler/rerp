// User-owned controller for handler 'list_ad_buys'.

use crate::handlers::list_ad_buys::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdBuy;

#[handler(ListAdBuysController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "accounts": [
    //         {
    //           "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
    //           "clicks": 148,
    //           "external_adset_ids": [
    //             "adset 9920…"
    //           ],
    //           "external_campaign_id": "cmp 2381…",
    //           "frequency": 4.4,
    //           "impressions": 7057,
    //           "platform": "META",
    //           "reach": 1604,
    //           "review": "APPROVED",
    //           "spend": "812.00",
    //           "status": "LIVE",
    //           "unsupported": []
    //         },
    //         {
    //           "account_id": "1dcd203d-8734-5d40-b43b-9bbcdd9758b5",
    //           "clicks": 51,
    //           "external_adset_ids": [
    //             "Customer Match signal only"
    //           ],
    //           "external_campaign_id": "cmp 23873…",
    //           "frequency": 3.6,
    //           "impressions": 3643,
    //           "platform": "GOOGLE_ADS",
    //           "reach": 1012,
    //           "review": "APPROVED",
    //           "spend": "418.00",
    //           "status": "LIVE",
    //           "unsupported": [
    //             "frequency cap (display only)",
    //             "dayparting on YouTube"
    //           ]
    //         },
    //         {
    //           "account_id": "6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa",
    //           "clicks": 15,
    //           "external_adset_ids": [
    //             "cmp 2201…"
    //           ],
    //           "external_campaign_id": "cmpgrp 5089…",
    //           "frequency": 4.1,
    //           "impressions": 1492,
    //           "platform": "LINKEDIN",
    //           "reach": 364,
    //           "review": "PENDING",
    //           "spend": "230.00",
    //           "status": "LIVE",
    //           "unsupported": [
    //             "cost cap (uses max CPM)"
    //           ]
    //         }
    //       ],
    //       "created_at": "2026-10-27T16:00:00Z",
    //       "created_by": "alex",
    //       "id": "3e845a81-8fde-5036-87dc-3c182dc80f7c",
    //       "name": "Warm-up · Black Friday early access",
    //       "purpose": "WARM_UP",
    //       "spec": {
    //         "bid": {
    //           "amount": "1250.00",
    //           "metric": "CPM",
    //           "strategy": "LOWEST_COST"
    //         },
    //         "budget": {
    //           "amount": "1800.00",
    //           "currency": "EUR",
    //           "pacing": "EVEN",
    //           "period": "LIFETIME"
    //         },
    //         "creative_ids": [
    //           "30f4c733-b7f5-51d2-b900-0be84807ebb2",
    //           "8767be08-2adf-5232-a68d-eefde450d501"
    //         ],
    //         "frequency_cap": {
    //           "impressions": 4,
    //           "per_days": 7
    //         },
    //         "objective": "WARM_UP",
    //         "placements": [],
    //         "schedule": {
    //           "dayparts": [
    //             {
    //               "days": [
    //                 "MON",
    //                 "TUE",
    //                 "WED"
    //               ],
    //               "from_hour": 7,
    //               "to_hour": 22
    //             }
    //           ],
    //           "end_at": "2026-11-18T16:00:00Z",
    //           "start_at": "2026-11-08T16:00:00Z",
    //           "timezone": "contact"
    //         },
    //         "targeting": {
    //           "audience_id": "b6d841b8-e819-50bb-ba55-f9f0c0e679bf",
    //           "devices": [
    //             "DESKTOP",
    //             "MOBILE",
    //             "TABLET"
    //           ],
    //           "exclude_audience_ids": [
    //             "e1668eef-08fd-5afb-bef2-1a2ac7dd3ba0"
    //           ],
    //           "geo": [
    //             "DE",
    //             "AT",
    //             "CH"
    //           ],
    //           "lookalike": false
    //         },
    //         "utm": {
    //           "utm_campaign": "bf-early-access",
    //           "utm_content": "{creative.id}",
    //           "utm_medium": "cpc",
    //           "utm_source": "{platform}"
    //         }
    //       },
    //       "status": "LIVE",
    //       "totals": {
    //         "attributed_leads": 9,
    //         "attributed_revenue": "1200.00",
    //         "clicks": 214,
    //         "reach": 2980,
    //         "spend": "1460.00"
    //       },
    //       "updated_at": "2026-11-05T15:00:00Z",
    //       "warm_up_for_campaign_id": "cf353d4a-972b-5b2f-8d04-b74803de4c99"
    //     },
    //     {
    //       "accounts": [
    //         {
    //           "account_id": "6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa",
    //           "clicks": 0,
    //           "external_adset_ids": [
    //             "cmp 2201…"
    //           ],
    //           "external_campaign_id": "cmpgrp 5089…",
    //           "frequency": 0,
    //           "impressions": 0,
    //           "platform": "LINKEDIN",
    //           "reach": 0,
    //           "review": "PENDING",
    //           "spend": "0.00",
    //           "status": "SCHEDULED",
    //           "unsupported": [
    //             "cost cap (uses max CPM)"
    //           ]
    //         }
    //       ],
    //       "created_at": "2026-10-28T16:00:00Z",
    //       "created_by": "alex",
    //       "id": "a169735e-6c23-5274-9166-74bdf0eeb9da",
    //       "name": "Warm-up · Webinar invite",
    //       "purpose": "WARM_UP",
    //       "spec": {
    //         "bid": {
    //           "amount": "1250.00",
    //           "metric": "CPM",
    //           "strategy": "LOWEST_COST"
    //         },
    //         "budget": {
    //           "amount": "600.00",
    //           "currency": "EUR",
    //           "pacing": "EVEN",
    //           "period": "LIFETIME"
    //         },
    //         "creative_ids": [
    //           "30f4c733-b7f5-51d2-b900-0be84807ebb2"
    //         ],
    //         "frequency_cap": {
    //           "impressions": 4,
    //           "per_days": 7
    //         },
    //         "objective": "WARM_UP",
    //         "placements": [],
    //         "schedule": {
    //           "dayparts": [
    //             {
    //               "days": [
    //                 "MON",
    //                 "TUE",
    //                 "WED"
    //               ],
    //               "from_hour": 7,
    //               "to_hour": 22
    //             }
    //           ],
    //           "end_at": "2026-11-18T16:00:00Z",
    //           "start_at": "2026-11-08T16:00:00Z",
    //           "timezone": "contact"
    //         },
    //         "targeting": {
    //           "audience_id": "bbab0796-23f0-59cb-841b-ce85c5b69aae",
    //           "devices": [
    //             "DESKTOP",
    //             "MOBILE",
    //             "TABLET"
    //           ],
    //           "exclude_audience_ids": [
    //             "e1668eef-08fd-5afb-bef2-1a2ac7dd3ba0"
    //           ],
    //           "geo": [
    //             "DE",
    //             "AT",
    //             "CH"
    //           ],
    //           "lookalike": false
    //         },
    //         "utm": {
    //           "utm_campaign": "warm-up",
    //           "utm_content": "{creative.id}",
    //           "utm_medium": "cpc",
    //           "utm_source": "{platform}"
    //         }
    //       },
    //       "status": "SCHEDULED",
    //       "totals": {
    //         "attributed_leads": 0,
    //         "attributed_revenue": "0.00",
    //         "clicks": 0,
    //         "reach": 0,
    //         "spend": "0.00"
    //       },
    //       "updated_at": "2026-11-05T14:00:00Z",
    //       "warm_up_for_campaign_id": "cf353d4a-972b-5b2f-8d04-b74803de4c99"
    //     },
    //     {
    //       "accounts": [
    //         {
    //           "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
    //           "clicks": 148,
    //           "external_adset_ids": [
    //             "adset 9920…"
    //           ],
    //           "external_campaign_id": "cmp 2381…",
    //           "frequency": 4.4,
    //           "impressions": 7057,
    //           "platform": "META",
    //           "reach": 1604,
    //           "review": "APPROVED",
    //           "spend": "812.00",
    //           "status": "LIVE",
    //           "unsupported": []
    //         },
    //         {
    //           "account_id": "1dcd203d-8734-5d40-b43b-9bbcdd9758b5",
    //           "clicks": 51,
    //           "external_adset_ids": [
    //             "Customer Match signal only"
    //           ],
    //           "external_campaign_id": "cmp 23873…",
    //           "frequency": 3.6,
    //           "impressions": 3643,
    //           "platform": "GOOGLE_ADS",
    //           "reach": 1012,
    //           "review": "APPROVED",
    //           "spend": "418.00",
    //           "status": "LIVE",
    //           "unsupported": [
    //             "frequency cap (display only)",
    //             "dayparting on YouTube"
    //           ]
    //         }
    //       ],
    //       "created_at": "2026-10-29T16:00:00Z",
    //       "created_by": "alex",
    //       "id": "5563ece1-e63e-554d-aeb6-32168c376cbd",
    //       "name": "Retarget · pricing visitors",
    //       "purpose": "RETARGET",
    //       "spec": {
    //         "bid": {
    //           "amount": "1250.00",
    //           "metric": "CPM",
    //           "strategy": "LOWEST_COST"
    //         },
    //         "budget": {
    //           "amount": "900.00",
    //           "currency": "EUR",
    //           "pacing": "EVEN",
    //           "period": "LIFETIME"
    //         },
    //         "creative_ids": [
    //           "30f4c733-b7f5-51d2-b900-0be84807ebb2"
    //         ],
    //         "frequency_cap": {
    //           "impressions": 4,
    //           "per_days": 7
    //         },
    //         "objective": "RETARGETING",
    //         "placements": [],
    //         "schedule": {
    //           "dayparts": [
    //             {
    //               "days": [
    //                 "MON",
    //                 "TUE",
    //                 "WED"
    //               ],
    //               "from_hour": 7,
    //               "to_hour": 22
    //             }
    //           ],
    //           "end_at": "2026-11-18T16:00:00Z",
    //           "start_at": "2026-11-08T16:00:00Z",
    //           "timezone": "contact"
    //         },
    //         "targeting": {
    //           "audience_id": "fa613142-3567-5941-ad4b-02a09f597962",
    //           "devices": [
    //             "DESKTOP",
    //             "MOBILE",
    //             "TABLET"
    //           ],
    //           "exclude_audience_ids": [
    //             "e1668eef-08fd-5afb-bef2-1a2ac7dd3ba0"
    //           ],
    //           "geo": [
    //             "DE",
    //             "AT",
    //             "CH"
    //           ],
    //           "lookalike": false
    //         },
    //         "utm": {
    //           "utm_campaign": "retarget",
    //           "utm_content": "{creative.id}",
    //           "utm_medium": "cpc",
    //           "utm_source": "{platform}"
    //         }
    //       },
    //       "status": "LIVE",
    //       "totals": {
    //         "attributed_leads": 21,
    //         "attributed_revenue": "7400.00",
    //         "clicks": 402,
    //         "reach": 1400,
    //         "spend": "620.00"
    //       },
    //       "updated_at": "2026-11-05T13:00:00Z",
    //       "warm_up_for_campaign_id": "7a0059fc-f220-5982-bdff-94e4e433840c"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 5
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
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
    },
    {
      "accounts": [
        {
          "account_id": "6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa",
          "clicks": 0,
          "external_adset_ids": [
            "cmp 2201…"
          ],
          "external_campaign_id": "cmpgrp 5089…",
          "frequency": 0,
          "impressions": 0,
          "platform": "LINKEDIN",
          "reach": 0,
          "review": "PENDING",
          "spend": "0.00",
          "status": "SCHEDULED",
          "unsupported": [
            "cost cap (uses max CPM)"
          ]
        }
      ],
      "created_at": "2026-10-28T16:00:00Z",
      "created_by": "alex",
      "id": "a169735e-6c23-5274-9166-74bdf0eeb9da",
      "name": "Warm-up · Webinar invite",
      "purpose": "WARM_UP",
      "spec": {
        "bid": {
          "amount": "1250.00",
          "metric": "CPM",
          "strategy": "LOWEST_COST"
        },
        "budget": {
          "amount": "600.00",
          "currency": "EUR",
          "pacing": "EVEN",
          "period": "LIFETIME"
        },
        "creative_ids": [
          "30f4c733-b7f5-51d2-b900-0be84807ebb2"
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
          "audience_id": "bbab0796-23f0-59cb-841b-ce85c5b69aae",
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
          "utm_campaign": "warm-up",
          "utm_content": "{creative.id}",
          "utm_medium": "cpc",
          "utm_source": "{platform}"
        }
      },
      "status": "SCHEDULED",
      "totals": {
        "attributed_leads": 0,
        "attributed_revenue": "0.00",
        "clicks": 0,
        "reach": 0,
        "spend": "0.00"
      },
      "updated_at": "2026-11-05T14:00:00Z",
      "warm_up_for_campaign_id": "cf353d4a-972b-5b2f-8d04-b74803de4c99"
    },
    {
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
        }
      ],
      "created_at": "2026-10-29T16:00:00Z",
      "created_by": "alex",
      "id": "5563ece1-e63e-554d-aeb6-32168c376cbd",
      "name": "Retarget · pricing visitors",
      "purpose": "RETARGET",
      "spec": {
        "bid": {
          "amount": "1250.00",
          "metric": "CPM",
          "strategy": "LOWEST_COST"
        },
        "budget": {
          "amount": "900.00",
          "currency": "EUR",
          "pacing": "EVEN",
          "period": "LIFETIME"
        },
        "creative_ids": [
          "30f4c733-b7f5-51d2-b900-0be84807ebb2"
        ],
        "frequency_cap": {
          "impressions": 4,
          "per_days": 7
        },
        "objective": "RETARGETING",
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
          "audience_id": "fa613142-3567-5941-ad4b-02a09f597962",
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
          "utm_campaign": "retarget",
          "utm_content": "{creative.id}",
          "utm_medium": "cpc",
          "utm_source": "{platform}"
        }
      },
      "status": "LIVE",
      "totals": {
        "attributed_leads": 21,
        "attributed_revenue": "7400.00",
        "clicks": 402,
        "reach": 1400,
        "spend": "620.00"
      },
      "updated_at": "2026-11-05T13:00:00Z",
      "warm_up_for_campaign_id": "7a0059fc-f220-5982-bdff-94e4e433840c"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 5
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<AdBuy>(serde_json::json!({"accounts":[{"account_id":"ded95315-6c77-51a0-b8ea-66b76fada21a","clicks":148,"external_adset_ids":["adset 9920…"],"external_campaign_id":"cmp 2381…","frequency":4.4,"impressions":7057,"platform":"META","reach":1604,"review":"APPROVED","spend":"812.00","status":"LIVE","unsupported":[]},{"account_id":"1dcd203d-8734-5d40-b43b-9bbcdd9758b5","clicks":51,"external_adset_ids":["Customer Match signal only"],"external_campaign_id":"cmp 23873…","frequency":3.6,"impressions":3643,"platform":"GOOGLE_ADS","reach":1012,"review":"APPROVED","spend":"418.00","status":"LIVE","unsupported":["frequency cap (display only)","dayparting on YouTube"]},{"account_id":"6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa","clicks":15,"external_adset_ids":["cmp 2201…"],"external_campaign_id":"cmpgrp 5089…","frequency":4.1,"impressions":1492,"platform":"LINKEDIN","reach":364,"review":"PENDING","spend":"230.00","status":"LIVE","unsupported":["cost cap (uses max CPM)"]}],"created_at":"2026-10-27T16:00:00Z","created_by":"alex","id":"3e845a81-8fde-5036-87dc-3c182dc80f7c","name":"Warm-up · Black Friday early access","purpose":"WARM_UP","spec":{"bid":{"amount":"1250.00","metric":"CPM","strategy":"LOWEST_COST"},"budget":{"amount":"1800.00","currency":"EUR","pacing":"EVEN","period":"LIFETIME"},"creative_ids":["30f4c733-b7f5-51d2-b900-0be84807ebb2","8767be08-2adf-5232-a68d-eefde450d501"],"frequency_cap":{"impressions":4,"per_days":7},"objective":"WARM_UP","placements":[],"schedule":{"dayparts":[{"days":["MON","TUE","WED"],"from_hour":7,"to_hour":22}],"end_at":"2026-11-18T16:00:00Z","start_at":"2026-11-08T16:00:00Z","timezone":"contact"},"targeting":{"audience_id":"b6d841b8-e819-50bb-ba55-f9f0c0e679bf","devices":["DESKTOP","MOBILE","TABLET"],"exclude_audience_ids":["e1668eef-08fd-5afb-bef2-1a2ac7dd3ba0"],"geo":["DE","AT","CH"],"lookalike":false},"utm":{"utm_campaign":"bf-early-access","utm_content":"{creative.id}","utm_medium":"cpc","utm_source":"{platform}"}},"status":"LIVE","totals":{"attributed_leads":9,"attributed_revenue":"1200.00","clicks":214,"reach":2980,"spend":"1460.00"},"updated_at":"2026-11-05T15:00:00Z","warm_up_for_campaign_id":"cf353d4a-972b-5b2f-8d04-b74803de4c99"})).unwrap_or_default(), serde_json::from_value::<AdBuy>(serde_json::json!({"accounts":[{"account_id":"6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa","clicks":0,"external_adset_ids":["cmp 2201…"],"external_campaign_id":"cmpgrp 5089…","frequency":0,"impressions":0,"platform":"LINKEDIN","reach":0,"review":"PENDING","spend":"0.00","status":"SCHEDULED","unsupported":["cost cap (uses max CPM)"]}],"created_at":"2026-10-28T16:00:00Z","created_by":"alex","id":"a169735e-6c23-5274-9166-74bdf0eeb9da","name":"Warm-up · Webinar invite","purpose":"WARM_UP","spec":{"bid":{"amount":"1250.00","metric":"CPM","strategy":"LOWEST_COST"},"budget":{"amount":"600.00","currency":"EUR","pacing":"EVEN","period":"LIFETIME"},"creative_ids":["30f4c733-b7f5-51d2-b900-0be84807ebb2"],"frequency_cap":{"impressions":4,"per_days":7},"objective":"WARM_UP","placements":[],"schedule":{"dayparts":[{"days":["MON","TUE","WED"],"from_hour":7,"to_hour":22}],"end_at":"2026-11-18T16:00:00Z","start_at":"2026-11-08T16:00:00Z","timezone":"contact"},"targeting":{"audience_id":"bbab0796-23f0-59cb-841b-ce85c5b69aae","devices":["DESKTOP","MOBILE","TABLET"],"exclude_audience_ids":["e1668eef-08fd-5afb-bef2-1a2ac7dd3ba0"],"geo":["DE","AT","CH"],"lookalike":false},"utm":{"utm_campaign":"warm-up","utm_content":"{creative.id}","utm_medium":"cpc","utm_source":"{platform}"}},"status":"SCHEDULED","totals":{"attributed_leads":0,"attributed_revenue":"0.00","clicks":0,"reach":0,"spend":"0.00"},"updated_at":"2026-11-05T14:00:00Z","warm_up_for_campaign_id":"cf353d4a-972b-5b2f-8d04-b74803de4c99"})).unwrap_or_default(), serde_json::from_value::<AdBuy>(serde_json::json!({"accounts":[{"account_id":"ded95315-6c77-51a0-b8ea-66b76fada21a","clicks":148,"external_adset_ids":["adset 9920…"],"external_campaign_id":"cmp 2381…","frequency":4.4,"impressions":7057,"platform":"META","reach":1604,"review":"APPROVED","spend":"812.00","status":"LIVE","unsupported":[]},{"account_id":"1dcd203d-8734-5d40-b43b-9bbcdd9758b5","clicks":51,"external_adset_ids":["Customer Match signal only"],"external_campaign_id":"cmp 23873…","frequency":3.6,"impressions":3643,"platform":"GOOGLE_ADS","reach":1012,"review":"APPROVED","spend":"418.00","status":"LIVE","unsupported":["frequency cap (display only)","dayparting on YouTube"]}],"created_at":"2026-10-29T16:00:00Z","created_by":"alex","id":"5563ece1-e63e-554d-aeb6-32168c376cbd","name":"Retarget · pricing visitors","purpose":"RETARGET","spec":{"bid":{"amount":"1250.00","metric":"CPM","strategy":"LOWEST_COST"},"budget":{"amount":"900.00","currency":"EUR","pacing":"EVEN","period":"LIFETIME"},"creative_ids":["30f4c733-b7f5-51d2-b900-0be84807ebb2"],"frequency_cap":{"impressions":4,"per_days":7},"objective":"RETARGETING","placements":[],"schedule":{"dayparts":[{"days":["MON","TUE","WED"],"from_hour":7,"to_hour":22}],"end_at":"2026-11-18T16:00:00Z","start_at":"2026-11-08T16:00:00Z","timezone":"contact"},"targeting":{"audience_id":"fa613142-3567-5941-ad4b-02a09f597962","devices":["DESKTOP","MOBILE","TABLET"],"exclude_audience_ids":["e1668eef-08fd-5afb-bef2-1a2ac7dd3ba0"],"geo":["DE","AT","CH"],"lookalike":false},"utm":{"utm_campaign":"retarget","utm_content":"{creative.id}","utm_medium":"cpc","utm_source":"{platform}"}},"status":"LIVE","totals":{"attributed_leads":21,"attributed_revenue":"7400.00","clicks":402,"reach":1400,"spend":"620.00"},"updated_at":"2026-11-05T13:00:00Z","warm_up_for_campaign_id":"7a0059fc-f220-5982-bdff-94e4e433840c"})).unwrap_or_default()],limit: 25,page: 1,total: 5,
    })
}
