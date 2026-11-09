// User-owned controller for handler 'get_quarterly_forecast'.

use crate::handlers::get_quarterly_forecast::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ForecastEntry;

#[handler(GetQuarterlyForecastController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "accuracy_last_3_months": 12.5,
    //   "items": [
    //     {
    //       "actual_close_revenue": "1250.00",
    //       "confidence": 12.5,
    //       "expected_close_revenue": "1250.00",
    //       "period": "Period",
    //       "variance": "1250.00",
    //       "weighted_pipeline": "1250.00"
    //     }
    //   ],
    //   "limit": 25,
    //   "no_close_date": {},
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "accuracy_last_3_months": 12.5,
  "items": [
    {
      "actual_close_revenue": "1250.00",
      "confidence": 12.5,
      "expected_close_revenue": "1250.00",
      "period": "Period",
      "variance": "1250.00",
      "weighted_pipeline": "1250.00"
    }
  ],
  "limit": 25,
  "no_close_date": {},
  "page": 1,
  "total": 2
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        accuracy_last_3_months: Some(12.5),items: vec![serde_json::from_value::<ForecastEntry>(serde_json::json!({"actual_close_revenue":"1250.00","confidence":12.5,"expected_close_revenue":"1250.00","period":"Period","variance":"1250.00","weighted_pipeline":"1250.00"})).unwrap_or_default()],limit: 25,no_close_date: Some(serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)),page: 1,total: 2,
    })
}
