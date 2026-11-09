// Implementation stub for handler 'get_automation_overview'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_insights_gen::handlers::get_automation_overview::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_insights_gen::handlers::types::AutomationMessageRow;
#[allow(unused_imports)]
use rerp_crm_insights_gen::handlers::types::NamedSeries;

#[handler(GetAutomationOverviewController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let id = req.inner.id;// let from = req.inner.from;// let to = req.inner.to;// let granularity = req.inner.granularity;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        click_rate: None, // TODO: Set from your business logic
        completion_rate: 0.855, // TODO: Set from your business logic
        contacts_active: None, // TODO: Set from your business logic
        contacts_finished: 1402, // TODO: Set from your business logic
        contacts_total: 1640, // TODO: Set from your business logic
        email_sent_rate: None, // TODO: Set from your business logic
        emails_sent: 6120, // TODO: Set from your business logic
        messages: vec![serde_json::from_value::<AutomationMessageRow>(serde_json::json!({"click_rate":0.129,"last_sent_at":"2026-11-05T16:00:00Z","name":"Day 0 · Welcome","open_rate":0.66,"revenue":"0.00","sends":1640,"step_id":"23d73cc0-2fda-5946-a568-c94ed4a4b6fb","unique_clicks":211,"unique_opens":1082})).unwrap_or_default(), serde_json::from_value::<AutomationMessageRow>(serde_json::json!({"click_rate":0.103,"last_sent_at":"2026-11-05T16:00:00Z","name":"Day 1 · Getting started","open_rate":0.55,"revenue":"0.00","sends":1598,"step_id":"ef70b030-e13c-5bc8-8e8b-8ed1237f31ea","unique_clicks":164,"unique_opens":879})).unwrap_or_default(), serde_json::from_value::<AutomationMessageRow>(serde_json::json!({"click_rate":0.087,"last_sent_at":"2026-11-05T16:00:00Z","name":"Day 3 · Tips for power users","open_rate":0.5,"revenue":"1200.00","sends":1402,"step_id":"34840ea3-97fc-5c23-9cbc-94fdecf47898","unique_clicks":122,"unique_opens":701})).unwrap_or_default()], // TODO: Set from your business logic
        name: "Welcome series (7 days)".to_string(), // TODO: Set from your business logic
        open_rate: None, // TODO: Set from your business logic
        revenue: None, // TODO: Set from your business logic
        series: None, // TODO: Set from your business logic
        unique_clicks: None, // TODO: Set from your business logic
        unique_opens: None, // TODO: Set from your business logic
        workflow_id: "22e49987-c27c-538a-9688-56d9ca17a7ec".to_string(), // TODO: Set from your business logic
    })
}
