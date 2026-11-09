// User-owned controller for handler 'get_ad_ltv'.

use crate::handlers::get_ad_ltv::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetAdLtvController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "by": "CHANNEL",
    //   "cohorts": [
    //     {
    //       "cac": "412.00",
    //       "cohort": "2026-08",
    //       "customers": 14,
    //       "key": "paid-social",
    //       "label": "Paid social",
    //       "ltv": [
    //         380,
    //         790,
    //         1240
    //       ],
    //       "payback_month": 2
    //     },
    //     {
    //       "cac": "388.00",
    //       "cohort": "2026-08",
    //       "customers": 9,
    //       "key": "paid-search",
    //       "label": "Paid search",
    //       "ltv": [
    //         310,
    //         640,
    //         980
    //       ],
    //       "payback_month": 2
    //     },
    //     {
    //       "cac": "96.00",
    //       "cohort": "2026-08",
    //       "customers": 22,
    //       "key": "email-warmed",
    //       "label": "Email (warmed)",
    //       "ltv": [
    //         520,
    //         980,
    //         1410
    //       ],
    //       "payback_month": 1
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "by": "CHANNEL",
  "cohorts": [
    {
      "cac": "412.00",
      "cohort": "2026-08",
      "customers": 14,
      "key": "paid-social",
      "label": "Paid social",
      "ltv": [
        380,
        790,
        1240
      ],
      "payback_month": 2
    },
    {
      "cac": "388.00",
      "cohort": "2026-08",
      "customers": 9,
      "key": "paid-search",
      "label": "Paid search",
      "ltv": [
        310,
        640,
        980
      ],
      "payback_month": 2
    },
    {
      "cac": "96.00",
      "cohort": "2026-08",
      "customers": 22,
      "key": "email-warmed",
      "label": "Email (warmed)",
      "ltv": [
        520,
        980,
        1410
      ],
      "payback_month": 1
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
        by: Some("CHANNEL".to_string()),cohorts: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{"cac":"412.00","cohort":"2026-08","customers":14,"key":"paid-social","label":"Paid social","ltv":[380,790,1240],"payback_month":2}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"cac":"388.00","cohort":"2026-08","customers":9,"key":"paid-search","label":"Paid search","ltv":[310,640,980],"payback_month":2}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"cac":"96.00","cohort":"2026-08","customers":22,"key":"email-warmed","label":"Email (warmed)","ltv":[520,980,1410],"payback_month":1}"#).unwrap_or(serde_json::Value::Null)]),
    })
}
