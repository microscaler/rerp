// User-owned controller for handler 'get_lead_source_effectiveness'.

use crate::handlers::get_lead_source_effectiveness::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::LeadSourceReport;

#[handler(GetLeadSourceEffectivenessController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "lead_count": 3,
    //       "opportunities": 3,
    //       "revenue": "1250.00",
    //       "source_id": "c709e57f-6f94-5c16-b15f-671d05f21521",
    //       "source_name": "Source name",
    //       "win_rate": 12.5,
    //       "won_deals": 3
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
      "lead_count": 3,
      "opportunities": 3,
      "revenue": "1250.00",
      "source_id": "c709e57f-6f94-5c16-b15f-671d05f21521",
      "source_name": "Source name",
      "win_rate": 12.5,
      "won_deals": 3
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
        items: vec![serde_json::from_value::<LeadSourceReport>(serde_json::json!({"lead_count":3,"opportunities":3,"revenue":"1250.00","source_id":"c709e57f-6f94-5c16-b15f-671d05f21521","source_name":"Source name","win_rate":12.5,"won_deals":3})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
