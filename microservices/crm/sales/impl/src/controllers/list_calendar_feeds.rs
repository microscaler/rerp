// Implementation stub for handler 'list_calendar_feeds'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_sales_gen::handlers::list_calendar_feeds::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_sales_gen::handlers::types::CalendarFeed;

#[handler(ListCalendarFeedsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let search = req.inner.search;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<CalendarFeed>(serde_json::json!({"assignee_ids":["alex"],"created_at":"2026-11-03T16:00:00Z","id":"da18346b-1f46-592f-ba4b-9d8012f04337","include_alerts":true,"name":"Sales tasks","pipeline_ids":["d9a2022e-96ff-53a3-a94d-2b169424fd31"],"scope":"DEALS","token":"55e9e0da0f6b5bac785a4e086d4672cd","url":"https://crm.acme.example/api/v1/crm/tasks/public/calendars/55e9e0da0f6b5bac785a4e086d4672cd.ics","user_id":"alex"})).unwrap_or_default(), serde_json::from_value::<CalendarFeed>(serde_json::json!({"assignee_ids":[],"created_at":"2026-09-26T16:00:00Z","id":"e094760a-c9e6-5722-a46b-a81f93262000","include_alerts":false,"name":"Everything","pipeline_ids":[],"scope":"ALL","token":"a91c2f0e7b8d4c6a9e1f3b5d7c9e04f1","url":"https://crm.acme.example/api/v1/crm/tasks/public/calendars/a91c2f0e7b8d4c6a9e1f3b5d7c9e04f1.ics","user_id":"alex"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 2, // TODO: Set from your business logic
    })
}
