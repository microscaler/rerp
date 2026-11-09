// User-owned controller for handler 'get_deal_forecast'.

use crate::handlers::get_deal_forecast::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ForecastAccuracyEntry;
#[allow(unused_imports)]
use crate::handlers::types::ForecastBasis;
#[allow(unused_imports)]
use crate::handlers::types::ForecastPeriod;

#[handler(GetDealForecastController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "accuracy": [
    //     {
    //       "actual": "84100.00",
    //       "expected": "92000.00",
    //       "period": "2026-10",
    //       "variance": "-7900.00",
    //       "variance_pct": -8.6
    //     },
    //     {
    //       "actual": "78300.00",
    //       "expected": "71500.00",
    //       "period": "2026-09",
    //       "variance": "6800.00",
    //       "variance_pct": 9.5
    //     },
    //     {
    //       "actual": "58200.00",
    //       "expected": "64000.00",
    //       "period": "2026-08",
    //       "variance": "-5800.00",
    //       "variance_pct": -9.1
    //     }
    //   ],
    //   "basis": "STAGE_PROBABILITY",
    //   "currency": "EUR",
    //   "generated_at": "2026-11-05T16:00:00Z",
    //   "granularity": "MONTH",
    //   "periods": [
    //     {
    //       "deals": [],
    //       "forecast": "151300.00",
    //       "open_count": 14,
    //       "open_value": "188400.00",
    //       "period": "2026-11",
    //       "quota": "200000.00",
    //       "weighted_value": "96100.00",
    //       "won_value": "55200.00"
    //     },
    //     {
    //       "deals": [],
    //       "forecast": "61800.00",
    //       "open_count": 12,
    //       "open_value": "141200.00",
    //       "period": "2026-12",
    //       "quota": "180000.00",
    //       "weighted_value": "61800.00",
    //       "won_value": "0.00"
    //     },
    //     {
    //       "deals": [],
    //       "forecast": "24300.00",
    //       "open_count": 9,
    //       "open_value": "82700.00",
    //       "period": "2027-01",
    //       "quota": "150000.00",
    //       "weighted_value": "24300.00",
    //       "won_value": "0.00"
    //     }
    //   ],
    //   "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "accuracy": [
    {
      "actual": "84100.00",
      "expected": "92000.00",
      "period": "2026-10",
      "variance": "-7900.00",
      "variance_pct": -8.6
    },
    {
      "actual": "78300.00",
      "expected": "71500.00",
      "period": "2026-09",
      "variance": "6800.00",
      "variance_pct": 9.5
    },
    {
      "actual": "58200.00",
      "expected": "64000.00",
      "period": "2026-08",
      "variance": "-5800.00",
      "variance_pct": -9.1
    }
  ],
  "basis": "STAGE_PROBABILITY",
  "currency": "EUR",
  "generated_at": "2026-11-05T16:00:00Z",
  "granularity": "MONTH",
  "periods": [
    {
      "deals": [],
      "forecast": "151300.00",
      "open_count": 14,
      "open_value": "188400.00",
      "period": "2026-11",
      "quota": "200000.00",
      "weighted_value": "96100.00",
      "won_value": "55200.00"
    },
    {
      "deals": [],
      "forecast": "61800.00",
      "open_count": 12,
      "open_value": "141200.00",
      "period": "2026-12",
      "quota": "180000.00",
      "weighted_value": "61800.00",
      "won_value": "0.00"
    },
    {
      "deals": [],
      "forecast": "24300.00",
      "open_count": 9,
      "open_value": "82700.00",
      "period": "2027-01",
      "quota": "150000.00",
      "weighted_value": "24300.00",
      "won_value": "0.00"
    }
  ],
  "pipeline_id": "d9a2022e-96ff-53a3-a94d-2b169424fd31"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        accuracy: Some(vec![serde_json::from_value::<ForecastAccuracyEntry>(serde_json::json!({"actual":"84100.00","expected":"92000.00","period":"2026-10","variance":"-7900.00","variance_pct":-8.6})).unwrap_or_default(), serde_json::from_value::<ForecastAccuracyEntry>(serde_json::json!({"actual":"78300.00","expected":"71500.00","period":"2026-09","variance":"6800.00","variance_pct":9.5})).unwrap_or_default(), serde_json::from_value::<ForecastAccuracyEntry>(serde_json::json!({"actual":"58200.00","expected":"64000.00","period":"2026-08","variance":"-5800.00","variance_pct":-9.1})).unwrap_or_default()]),basis: serde_json::from_value::<ForecastBasis>(serde_json::Value::String("STAGE_PROBABILITY".to_string())).unwrap_or_default(),currency: Some("EUR".to_string()),generated_at: Some("2026-11-05T16:00:00Z".to_string()),granularity: "MONTH".to_string(),periods: vec![serde_json::from_value::<ForecastPeriod>(serde_json::json!({"deals":[],"forecast":"151300.00","open_count":14,"open_value":"188400.00","period":"2026-11","quota":"200000.00","weighted_value":"96100.00","won_value":"55200.00"})).unwrap_or_default(), serde_json::from_value::<ForecastPeriod>(serde_json::json!({"deals":[],"forecast":"61800.00","open_count":12,"open_value":"141200.00","period":"2026-12","quota":"180000.00","weighted_value":"61800.00","won_value":"0.00"})).unwrap_or_default(), serde_json::from_value::<ForecastPeriod>(serde_json::json!({"deals":[],"forecast":"24300.00","open_count":9,"open_value":"82700.00","period":"2027-01","quota":"150000.00","weighted_value":"24300.00","won_value":"0.00"})).unwrap_or_default()],pipeline_id: "d9a2022e-96ff-53a3-a94d-2b169424fd31".to_string(),
    })
}
