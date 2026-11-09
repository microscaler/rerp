// User-owned controller for handler 'get_campaign_report'.

use crate::handlers::get_campaign_report::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::CampaignVariant;

#[handler(GetCampaignReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        anonymous_open_rate: Some(1.5),
        audience: Some(Default::default()),
        bounced: 42,
        campaign_id: "example".to_string(),
        click_rate: Some(1.5),
        click_to_open_rate: Some(1.5),
        clicked: 42,
        complained: 42,
        delivered: 42,
        email_clients: Some(vec![]),
        forwarded: Some(42),
        from: Some("example".to_string()),
        links: Some(vec![]),
        open_rate: Some(1.5),
        opened: 42,
        opens_by_date: Some(vec![]),
        opens_by_hour: Some(vec![]),
        opens_by_location: Some(vec![]),
        replied: Some(42),
        revenue: Some(rust_decimal::Decimal::new(12345, 2)),
        sent: 42,
        sent_at: Some("example".to_string()),
        social_shares: Some(vec![]),
        subject: Some("example".to_string()),
        total_clicks: Some(42),
        total_opens: Some(42),
        unsubscribed: 42,
        unverified_opens: Some(42),
        updated: Some(42),
        variants: Some(vec![]),
        verified_opens: Some(42),
    })
}
