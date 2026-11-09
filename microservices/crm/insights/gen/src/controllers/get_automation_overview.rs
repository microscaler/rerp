// User-owned controller for handler 'get_automation_overview'.

use crate::handlers::get_automation_overview::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AutomationMessageRow;
#[allow(unused_imports)]
use crate::handlers::types::NamedSeries;

#[handler(GetAutomationOverviewController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        click_rate: Some(1.5),
        completion_rate: 1.5,
        contacts_finished: 42,
        contacts_total: 42,
        email_sent_rate: Some(1.5),
        emails_sent: 42,
        messages: vec![],
        name: "example".to_string(),
        open_rate: Some(1.5),
        revenue: Some(rust_decimal::Decimal::new(12345, 2)),
        series: Some(vec![]),
        unique_clicks: Some(42),
        unique_opens: Some(42),
        workflow_id: "example".to_string(),
    })
}
