// User-owned controller for handler 'get_campaign_revenue'.

use crate::handlers::get_campaign_revenue::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetCampaignRevenueController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        average_deal_size: Some(1.5),
        total_opportunities: Some(42),
        total_revenue: Some(rust_decimal::Decimal::new(12345, 2)),
        total_won_deals: Some(42),
    })
}
