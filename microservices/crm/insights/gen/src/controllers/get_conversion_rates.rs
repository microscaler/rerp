// User-owned controller for handler 'get_conversion_rates'.

use crate::handlers::get_conversion_rates::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ConversionRate;

#[handler(GetConversionRatesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "lead_to_opp_rate": 12.5,
    //       "leads_created": 3,
    //       "opp_to_won_rate": 12.5,
    //       "opportunities_created": 3,
    //       "opportunities_lost": 3,
    //       "opportunities_won": 3,
    //       "overall_conversion_rate": 12.5,
    //       "period": "Period",
    //       "total_won_revenue": "1250.00"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "lead_to_opp_rate": 12.5,
      "leads_created": 3,
      "opp_to_won_rate": 12.5,
      "opportunities_created": 3,
      "opportunities_lost": 3,
      "opportunities_won": 3,
      "overall_conversion_rate": 12.5,
      "period": "Period",
      "total_won_revenue": "1250.00"
    }
  ],
  "limit": 25,
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
        items: vec![serde_json::from_value::<ConversionRate>(serde_json::json!({"lead_to_opp_rate":12.5,"leads_created":3,"opp_to_won_rate":12.5,"opportunities_created":3,"opportunities_lost":3,"opportunities_won":3,"overall_conversion_rate":12.5,"period":"Period","total_won_revenue":"1250.00"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
