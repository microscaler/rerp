// User-owned controller for handler 'get_campaign_leads'.

use crate::handlers::get_campaign_leads::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::CampaignLeadResult;

#[handler(GetCampaignLeadsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "created_at": "2026-11-05T16:00:00Z",
    //       "lead_id": "a46e2e3a-01f7-5c22-abcb-22698f556fda",
    //       "lead_name": "Lead name"
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
      "created_at": "2026-11-05T16:00:00Z",
      "lead_id": "a46e2e3a-01f7-5c22-abcb-22698f556fda",
      "lead_name": "Lead name"
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
        items: vec![serde_json::from_value::<CampaignLeadResult>(serde_json::json!({"created_at":"2026-11-05T16:00:00Z","lead_id":"a46e2e3a-01f7-5c22-abcb-22698f556fda","lead_name":"Lead name"})).unwrap_or_default()],limit: 25,page: 1,total: 2,
    })
}
