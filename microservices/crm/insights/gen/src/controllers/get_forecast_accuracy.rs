// User-owned controller for handler 'get_forecast_accuracy'.

use crate::handlers::get_forecast_accuracy::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ForecastAccuracyEntry;

#[handler(GetForecastAccuracyController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "actual": "1250.00",
    //       "expected": "1250.00",
    //       "period": "Period",
    //       "variance": "1250.00",
    //       "variance_pct": 12.5
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "actual": "1250.00",
      "expected": "1250.00",
      "period": "Period",
      "variance": "1250.00",
      "variance_pct": 12.5
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 2
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<ForecastAccuracyEntry>(serde_json::json!({"actual":"1250.00","expected":"1250.00","period":"Period","variance":"1250.00","variance_pct":12.5})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
