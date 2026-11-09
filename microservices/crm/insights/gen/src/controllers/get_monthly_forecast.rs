// User-owned controller for handler 'get_monthly_forecast'.

use crate::handlers::get_monthly_forecast::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ForecastEntry;

#[handler(GetMonthlyForecastController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "accuracy_last_3_months": 0.88,
    //   "items": [
    //     {
    //       "actual_close_revenue": "1250.00",
    //       "confidence": 12.5,
    //       "expected_close_revenue": "1250.00",
    //       "month": "2026-11",
    //       "open_deals": 14,
    //       "period": "Period",
    //       "pipeline_value": 188400,
    //       "target": 200000,
    //       "variance": "1250.00",
    //       "weighted": 96100,
    //       "weighted_pipeline": "1250.00",
    //       "won": 55200
    //     },
    //     {
    //       "actual_close_revenue": "1250.00",
    //       "confidence": 12.5,
    //       "expected_close_revenue": "1250.00",
    //       "month": "2026-12",
    //       "open_deals": 12,
    //       "period": "Period",
    //       "pipeline_value": 141200,
    //       "target": 180000,
    //       "variance": "1250.00",
    //       "weighted": 61800,
    //       "weighted_pipeline": "1250.00",
    //       "won": 0
    //     },
    //     {
    //       "actual_close_revenue": "1250.00",
    //       "confidence": 12.5,
    //       "expected_close_revenue": "1250.00",
    //       "month": "2027-01",
    //       "open_deals": 9,
    //       "period": "Period",
    //       "pipeline_value": 82700,
    //       "target": 150000,
    //       "variance": "1250.00",
    //       "weighted": 24300,
    //       "weighted_pipeline": "1250.00",
    //       "won": 0
    //     }
    //   ],
    //   "limit": 25,
    //   "no_close_date": {
    //     "count": 3,
    //     "value": 15000
    //   },
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "accuracy_last_3_months": 0.88,
  "items": [
    {
      "actual_close_revenue": "1250.00",
      "confidence": 12.5,
      "expected_close_revenue": "1250.00",
      "month": "2026-11",
      "open_deals": 14,
      "period": "Period",
      "pipeline_value": 188400,
      "target": 200000,
      "variance": "1250.00",
      "weighted": 96100,
      "weighted_pipeline": "1250.00",
      "won": 55200
    },
    {
      "actual_close_revenue": "1250.00",
      "confidence": 12.5,
      "expected_close_revenue": "1250.00",
      "month": "2026-12",
      "open_deals": 12,
      "period": "Period",
      "pipeline_value": 141200,
      "target": 180000,
      "variance": "1250.00",
      "weighted": 61800,
      "weighted_pipeline": "1250.00",
      "won": 0
    },
    {
      "actual_close_revenue": "1250.00",
      "confidence": 12.5,
      "expected_close_revenue": "1250.00",
      "month": "2027-01",
      "open_deals": 9,
      "period": "Period",
      "pipeline_value": 82700,
      "target": 150000,
      "variance": "1250.00",
      "weighted": 24300,
      "weighted_pipeline": "1250.00",
      "won": 0
    }
  ],
  "limit": 25,
  "no_close_date": {
    "count": 3,
    "value": 15000
  },
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
        accuracy_last_3_months: Some(0.88),items: vec![serde_json::from_value::<ForecastEntry>(serde_json::json!({"actual_close_revenue":"1250.00","confidence":12.5,"expected_close_revenue":"1250.00","month":"2026-11","open_deals":14,"period":"Period","pipeline_value":188400,"target":200000,"variance":"1250.00","weighted":96100,"weighted_pipeline":"1250.00","won":55200})).unwrap_or_default(), serde_json::from_value::<ForecastEntry>(serde_json::json!({"actual_close_revenue":"1250.00","confidence":12.5,"expected_close_revenue":"1250.00","month":"2026-12","open_deals":12,"period":"Period","pipeline_value":141200,"target":180000,"variance":"1250.00","weighted":61800,"weighted_pipeline":"1250.00","won":0})).unwrap_or_default(), serde_json::from_value::<ForecastEntry>(serde_json::json!({"actual_close_revenue":"1250.00","confidence":12.5,"expected_close_revenue":"1250.00","month":"2027-01","open_deals":9,"period":"Period","pipeline_value":82700,"target":150000,"variance":"1250.00","weighted":24300,"weighted_pipeline":"1250.00","won":0})).unwrap_or_default()],limit: 25,no_close_date: Some(serde_json::from_str::<serde_json::Value>(r#"{"count":3,"value":15000}"#).unwrap_or(serde_json::Value::Null)),page: 1,total: 2,
    })
}
