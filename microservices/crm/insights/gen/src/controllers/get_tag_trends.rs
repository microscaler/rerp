// User-owned controller for handler 'get_tag_trends'.

use crate::handlers::get_tag_trends::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::NameCount;
#[allow(unused_imports)]
use crate::handlers::types::NamedSeries;
#[allow(unused_imports)]
use crate::handlers::types::TagTrendRow;

#[handler(GetTagTrendsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "avg_per_day": 74.8,
    //   "most_used_in_range": [
    //     {
    //       "count": 1104,
    //       "name": "lead"
    //     },
    //     {
    //       "count": 612,
    //       "name": "trial"
    //     },
    //     {
    //       "count": 301,
    //       "name": "pricing-page"
    //     }
    //   ],
    //   "most_used_overall": [
    //     {
    //       "count": 6114,
    //       "name": "lead"
    //     },
    //     {
    //       "count": 1820,
    //       "name": "customer"
    //     },
    //     {
    //       "count": 1412,
    //       "name": "trial"
    //     }
    //   ],
    //   "series": [
    //     {
    //       "key": "lead",
    //       "label": "lead",
    //       "points": [
    //         {
    //           "date": "2026-10-07",
    //           "value": 30
    //         },
    //         {
    //           "date": "2026-10-08",
    //           "value": 37
    //         },
    //         {
    //           "date": "2026-10-09",
    //           "value": 33
    //         }
    //       ]
    //     },
    //     {
    //       "key": "trial",
    //       "label": "trial",
    //       "points": [
    //         {
    //           "date": "2026-10-07",
    //           "value": 14
    //         },
    //         {
    //           "date": "2026-10-08",
    //           "value": 21
    //         },
    //         {
    //           "date": "2026-10-09",
    //           "value": 17
    //         }
    //       ]
    //     },
    //     {
    //       "key": "customer",
    //       "label": "customer",
    //       "points": [
    //         {
    //           "date": "2026-10-07",
    //           "value": 2
    //         },
    //         {
    //           "date": "2026-10-08",
    //           "value": 9
    //         },
    //         {
    //           "date": "2026-10-09",
    //           "value": 5
    //         }
    //       ]
    //     }
    //   ],
    //   "tags": [
    //     {
    //       "applied_in_range": 1104,
    //       "avg_per_day": 35.6,
    //       "first_applied_at": "2026-01-02T00:00:00Z",
    //       "last_applied_at": "2026-11-05T16:00:00Z",
    //       "name": "lead",
    //       "tag_id": "9fcf4e9e-e339-5eef-8d3d-9086b582243e",
    //       "total_contacts": 6114
    //     },
    //     {
    //       "applied_in_range": 612,
    //       "avg_per_day": 19.7,
    //       "first_applied_at": "2026-02-14T00:00:00Z",
    //       "last_applied_at": "2026-11-05T16:00:00Z",
    //       "name": "trial",
    //       "tag_id": "9ea75ff5-0642-5599-9bf6-34f9bbcefd89",
    //       "total_contacts": 1412
    //     },
    //     {
    //       "applied_in_range": 301,
    //       "avg_per_day": 9.7,
    //       "first_applied_at": "2026-03-03T00:00:00Z",
    //       "last_applied_at": "2026-11-05T16:00:00Z",
    //       "name": "pricing-page",
    //       "tag_id": "83c2db54-32d0-50e6-a226-f6487560ac02",
    //       "total_contacts": 902
    //     }
    //   ],
    //   "tags_applied": 2318
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "avg_per_day": 74.8,
  "most_used_in_range": [
    {
      "count": 1104,
      "name": "lead"
    },
    {
      "count": 612,
      "name": "trial"
    },
    {
      "count": 301,
      "name": "pricing-page"
    }
  ],
  "most_used_overall": [
    {
      "count": 6114,
      "name": "lead"
    },
    {
      "count": 1820,
      "name": "customer"
    },
    {
      "count": 1412,
      "name": "trial"
    }
  ],
  "series": [
    {
      "key": "lead",
      "label": "lead",
      "points": [
        {
          "date": "2026-10-07",
          "value": 30
        },
        {
          "date": "2026-10-08",
          "value": 37
        },
        {
          "date": "2026-10-09",
          "value": 33
        }
      ]
    },
    {
      "key": "trial",
      "label": "trial",
      "points": [
        {
          "date": "2026-10-07",
          "value": 14
        },
        {
          "date": "2026-10-08",
          "value": 21
        },
        {
          "date": "2026-10-09",
          "value": 17
        }
      ]
    },
    {
      "key": "customer",
      "label": "customer",
      "points": [
        {
          "date": "2026-10-07",
          "value": 2
        },
        {
          "date": "2026-10-08",
          "value": 9
        },
        {
          "date": "2026-10-09",
          "value": 5
        }
      ]
    }
  ],
  "tags": [
    {
      "applied_in_range": 1104,
      "avg_per_day": 35.6,
      "first_applied_at": "2026-01-02T00:00:00Z",
      "last_applied_at": "2026-11-05T16:00:00Z",
      "name": "lead",
      "tag_id": "9fcf4e9e-e339-5eef-8d3d-9086b582243e",
      "total_contacts": 6114
    },
    {
      "applied_in_range": 612,
      "avg_per_day": 19.7,
      "first_applied_at": "2026-02-14T00:00:00Z",
      "last_applied_at": "2026-11-05T16:00:00Z",
      "name": "trial",
      "tag_id": "9ea75ff5-0642-5599-9bf6-34f9bbcefd89",
      "total_contacts": 1412
    },
    {
      "applied_in_range": 301,
      "avg_per_day": 9.7,
      "first_applied_at": "2026-03-03T00:00:00Z",
      "last_applied_at": "2026-11-05T16:00:00Z",
      "name": "pricing-page",
      "tag_id": "83c2db54-32d0-50e6-a226-f6487560ac02",
      "total_contacts": 902
    }
  ],
  "tags_applied": 2318
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        avg_per_day: 74.8,most_used_in_range: Some(vec![serde_json::from_value::<NameCount>(serde_json::json!({"count":1104,"name":"lead"})).unwrap_or_default(), serde_json::from_value::<NameCount>(serde_json::json!({"count":612,"name":"trial"})).unwrap_or_default(), serde_json::from_value::<NameCount>(serde_json::json!({"count":301,"name":"pricing-page"})).unwrap_or_default()]),most_used_overall: Some(vec![serde_json::from_value::<NameCount>(serde_json::json!({"count":6114,"name":"lead"})).unwrap_or_default(), serde_json::from_value::<NameCount>(serde_json::json!({"count":1820,"name":"customer"})).unwrap_or_default(), serde_json::from_value::<NameCount>(serde_json::json!({"count":1412,"name":"trial"})).unwrap_or_default()]),series: vec![serde_json::from_value::<NamedSeries>(serde_json::json!({"key":"lead","label":"lead","points":[{"date":"2026-10-07","value":30},{"date":"2026-10-08","value":37},{"date":"2026-10-09","value":33}]})).unwrap_or_default(), serde_json::from_value::<NamedSeries>(serde_json::json!({"key":"trial","label":"trial","points":[{"date":"2026-10-07","value":14},{"date":"2026-10-08","value":21},{"date":"2026-10-09","value":17}]})).unwrap_or_default(), serde_json::from_value::<NamedSeries>(serde_json::json!({"key":"customer","label":"customer","points":[{"date":"2026-10-07","value":2},{"date":"2026-10-08","value":9},{"date":"2026-10-09","value":5}]})).unwrap_or_default()],tags: vec![serde_json::from_value::<TagTrendRow>(serde_json::json!({"applied_in_range":1104,"avg_per_day":35.6,"first_applied_at":"2026-01-02T00:00:00Z","last_applied_at":"2026-11-05T16:00:00Z","name":"lead","tag_id":"9fcf4e9e-e339-5eef-8d3d-9086b582243e","total_contacts":6114})).unwrap_or_default(), serde_json::from_value::<TagTrendRow>(serde_json::json!({"applied_in_range":612,"avg_per_day":19.7,"first_applied_at":"2026-02-14T00:00:00Z","last_applied_at":"2026-11-05T16:00:00Z","name":"trial","tag_id":"9ea75ff5-0642-5599-9bf6-34f9bbcefd89","total_contacts":1412})).unwrap_or_default(), serde_json::from_value::<TagTrendRow>(serde_json::json!({"applied_in_range":301,"avg_per_day":9.7,"first_applied_at":"2026-03-03T00:00:00Z","last_applied_at":"2026-11-05T16:00:00Z","name":"pricing-page","tag_id":"83c2db54-32d0-50e6-a226-f6487560ac02","total_contacts":902})).unwrap_or_default()],tags_applied: 2318,
    })
}
