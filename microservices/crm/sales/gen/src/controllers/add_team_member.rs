// User-owned controller for handler 'add_team_member'.

use crate::handlers::add_team_member::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(AddTeamMemberController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        assignment_domain: Some("example".to_string()),
        assignment_enabled: true,
        assignment_max: 42,
        assignment_optout: Some(true),
        color: Some(42),
        crm_team_id: "example".to_string(),
        id: "example".to_string(),
        lead_day_count: Some(42),
        lead_month_count: Some(42),
        quota_monthly: Some(1.5),
        quota_quarterly: Some(1.5),
        quota_yearly: Some(1.5),
        user_id: "example".to_string(),
        user_name: Some("example".to_string()),
    })
}
