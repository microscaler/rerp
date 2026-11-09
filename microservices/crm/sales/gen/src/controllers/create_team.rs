// User-owned controller for handler 'create_team'.

use crate::handlers::create_team::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(CreateTeamController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        active_lead_count: Some(42),
        alias_name: Some("example".to_string()),
        assignment_auto_enabled: Some(true),
        assignment_domain: Some("example".to_string()),
        assignment_enabled: true,
        assignment_max: Some(42),
        assignment_optout: Some(true),
        create_date: Some("example".to_string()),
        description: Some("example".to_string()),
        emoji: Some("example".to_string()),
        id: "example".to_string(),
        member_count: Some(42),
        name: "example".to_string(),
        privacy_security: Some("example".to_string()),
        total_revenue: Some(rust_decimal::Decimal::new(12345, 2)),
        use_leads: Some(true),
        use_opportunities: Some(true),
    })
}
