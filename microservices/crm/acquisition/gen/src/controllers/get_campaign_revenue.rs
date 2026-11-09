// User-owned controller for handler 'get_campaign_revenue'.

use crate::handlers::get_campaign_revenue::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetCampaignRevenueController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "average_deal_size": "1250.00",
    //   "total_opportunities": 3,
    //   "total_revenue": "1250.00",
    //   "total_won_deals": 3
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "average_deal_size": "1250.00",
  "total_opportunities": 3,
  "total_revenue": "1250.00",
  "total_won_deals": 3
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        average_deal_size: Some("1250.00".to_string()),
        total_opportunities: Some(3),
        total_revenue: Some("1250.00".to_string()),
        total_won_deals: Some(3),
    })
}
