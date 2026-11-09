// User-owned controller for handler 'get_contact_performance'.

use crate::handlers::get_contact_performance::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ContactCounts;
#[allow(unused_imports)]
use crate::handlers::types::ContactGroupRow;
#[allow(unused_imports)]
use crate::handlers::types::NamedSeries;

#[handler(GetContactPerformanceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "current": {
    //     "active": 16104,
    //     "inactive": 2316,
    //     "total": 18420
    //   },
    //   "granularity": "DAY",
    //   "previous": {
    //     "active": 15564,
    //     "inactive": 2244,
    //     "total": 17808
    //   },
    //   "top_lists": [
    //     {
    //       "active": 9201,
    //       "growth_pct": 2.8,
    //       "id": "09f5b2f9-2adc-5edd-a6fd-c20675680671",
    //       "inactive": 669,
    //       "name": "Newsletter",
    //       "total": 9870
    //     },
    //     {
    //       "active": 1802,
    //       "growth_pct": 2.2,
    //       "id": "f84269dd-7dba-5a82-ad63-986af03f39c1",
    //       "inactive": 18,
    //       "name": "Customers",
    //       "total": 1820
    //     },
    //     {
    //       "active": 3022,
    //       "growth_pct": 7.7,
    //       "id": "06b96791-3940-54b0-b719-717deb92aad3",
    //       "inactive": 188,
    //       "name": "Webinars",
    //       "total": 3210
    //     }
    //   ],
    //   "top_tags": [
    //     {
    //       "active": 5804,
    //       "growth_pct": 5.1,
    //       "id": "9fcf4e9e-e339-5eef-8d3d-9086b582243e",
    //       "inactive": 310,
    //       "name": "lead",
    //       "total": 6114
    //     },
    //     {
    //       "active": 1802,
    //       "growth_pct": 2.2,
    //       "id": "f088dbd4-36e8-50b1-be4c-5714acf532d6",
    //       "inactive": 18,
    //       "name": "customer",
    //       "total": 1820
    //     },
    //     {
    //       "active": 1390,
    //       "growth_pct": 12.0,
    //       "id": "9ea75ff5-0642-5599-9bf6-34f9bbcefd89",
    //       "inactive": 22,
    //       "name": "trial",
    //       "total": 1412
    //     }
    //   ],
    //   "trend": [
    //     {
    //       "key": "total",
    //       "label": "Total",
    //       "points": [
    //         {
    //           "date": "2026-10-07",
    //           "value": 17808
    //         },
    //         {
    //           "date": "2026-10-08",
    //           "value": 17829
    //         },
    //         {
    //           "date": "2026-10-09",
    //           "value": 17850
    //         }
    //       ]
    //     },
    //     {
    //       "key": "active",
    //       "label": "Active",
    //       "points": [
    //         {
    //           "date": "2026-10-07",
    //           "value": 15564
    //         },
    //         {
    //           "date": "2026-10-08",
    //           "value": 15582
    //         },
    //         {
    //           "date": "2026-10-09",
    //           "value": 15600
    //         }
    //       ]
    //     },
    //     {
    //       "key": "inactive",
    //       "label": "Inactive",
    //       "points": [
    //         {
    //           "date": "2026-10-07",
    //           "value": 2244
    //         },
    //         {
    //           "date": "2026-10-08",
    //           "value": 2247
    //         },
    //         {
    //           "date": "2026-10-09",
    //           "value": 2250
    //         }
    //       ]
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "current": {
    "active": 16104,
    "inactive": 2316,
    "total": 18420
  },
  "granularity": "DAY",
  "previous": {
    "active": 15564,
    "inactive": 2244,
    "total": 17808
  },
  "top_lists": [
    {
      "active": 9201,
      "growth_pct": 2.8,
      "id": "09f5b2f9-2adc-5edd-a6fd-c20675680671",
      "inactive": 669,
      "name": "Newsletter",
      "total": 9870
    },
    {
      "active": 1802,
      "growth_pct": 2.2,
      "id": "f84269dd-7dba-5a82-ad63-986af03f39c1",
      "inactive": 18,
      "name": "Customers",
      "total": 1820
    },
    {
      "active": 3022,
      "growth_pct": 7.7,
      "id": "06b96791-3940-54b0-b719-717deb92aad3",
      "inactive": 188,
      "name": "Webinars",
      "total": 3210
    }
  ],
  "top_tags": [
    {
      "active": 5804,
      "growth_pct": 5.1,
      "id": "9fcf4e9e-e339-5eef-8d3d-9086b582243e",
      "inactive": 310,
      "name": "lead",
      "total": 6114
    },
    {
      "active": 1802,
      "growth_pct": 2.2,
      "id": "f088dbd4-36e8-50b1-be4c-5714acf532d6",
      "inactive": 18,
      "name": "customer",
      "total": 1820
    },
    {
      "active": 1390,
      "growth_pct": 12.0,
      "id": "9ea75ff5-0642-5599-9bf6-34f9bbcefd89",
      "inactive": 22,
      "name": "trial",
      "total": 1412
    }
  ],
  "trend": [
    {
      "key": "total",
      "label": "Total",
      "points": [
        {
          "date": "2026-10-07",
          "value": 17808
        },
        {
          "date": "2026-10-08",
          "value": 17829
        },
        {
          "date": "2026-10-09",
          "value": 17850
        }
      ]
    },
    {
      "key": "active",
      "label": "Active",
      "points": [
        {
          "date": "2026-10-07",
          "value": 15564
        },
        {
          "date": "2026-10-08",
          "value": 15582
        },
        {
          "date": "2026-10-09",
          "value": 15600
        }
      ]
    },
    {
      "key": "inactive",
      "label": "Inactive",
      "points": [
        {
          "date": "2026-10-07",
          "value": 2244
        },
        {
          "date": "2026-10-08",
          "value": 2247
        },
        {
          "date": "2026-10-09",
          "value": 2250
        }
      ]
    }
  ]
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        current: serde_json::from_value::<ContactCounts>(serde_json::json!({"active":16104,"inactive":2316,"total":18420})).unwrap_or_default(),granularity: Some("DAY".to_string()),previous: serde_json::from_value::<ContactCounts>(serde_json::json!({"active":15564,"inactive":2244,"total":17808})).unwrap_or_default(),top_lists: vec![serde_json::from_value::<ContactGroupRow>(serde_json::json!({"active":9201,"growth_pct":2.8,"id":"09f5b2f9-2adc-5edd-a6fd-c20675680671","inactive":669,"name":"Newsletter","total":9870})).unwrap_or_default(), serde_json::from_value::<ContactGroupRow>(serde_json::json!({"active":1802,"growth_pct":2.2,"id":"f84269dd-7dba-5a82-ad63-986af03f39c1","inactive":18,"name":"Customers","total":1820})).unwrap_or_default(), serde_json::from_value::<ContactGroupRow>(serde_json::json!({"active":3022,"growth_pct":7.7,"id":"06b96791-3940-54b0-b719-717deb92aad3","inactive":188,"name":"Webinars","total":3210})).unwrap_or_default()],top_tags: vec![serde_json::from_value::<ContactGroupRow>(serde_json::json!({"active":5804,"growth_pct":5.1,"id":"9fcf4e9e-e339-5eef-8d3d-9086b582243e","inactive":310,"name":"lead","total":6114})).unwrap_or_default(), serde_json::from_value::<ContactGroupRow>(serde_json::json!({"active":1802,"growth_pct":2.2,"id":"f088dbd4-36e8-50b1-be4c-5714acf532d6","inactive":18,"name":"customer","total":1820})).unwrap_or_default(), serde_json::from_value::<ContactGroupRow>(serde_json::json!({"active":1390,"growth_pct":12.0,"id":"9ea75ff5-0642-5599-9bf6-34f9bbcefd89","inactive":22,"name":"trial","total":1412})).unwrap_or_default()],trend: vec![serde_json::from_value::<NamedSeries>(serde_json::json!({"key":"total","label":"Total","points":[{"date":"2026-10-07","value":17808},{"date":"2026-10-08","value":17829},{"date":"2026-10-09","value":17850}]})).unwrap_or_default(), serde_json::from_value::<NamedSeries>(serde_json::json!({"key":"active","label":"Active","points":[{"date":"2026-10-07","value":15564},{"date":"2026-10-08","value":15582},{"date":"2026-10-09","value":15600}]})).unwrap_or_default(), serde_json::from_value::<NamedSeries>(serde_json::json!({"key":"inactive","label":"Inactive","points":[{"date":"2026-10-07","value":2244},{"date":"2026-10-08","value":2247},{"date":"2026-10-09","value":2250}]})).unwrap_or_default()],
    })
}
