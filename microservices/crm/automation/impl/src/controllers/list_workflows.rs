// Implementation stub for handler 'list_workflows'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_automation_gen::handlers::list_workflows::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_automation_gen::handlers::types::Workflow;

#[handler(ListWorkflowsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let search = req.inner.search;// let active = req.inner.active;// let trigger_type = req.inner.trigger_type;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Workflow>(serde_json::json!({"active_enrolments":1204,"completed_enrolments":8311,"created_at":"2026-06-08T16:00:00Z","description":"","entry_step_id":"f1189b89-3e27-5826-bc93-e33f7450d394","goal_rate":0.23,"id":"22e49987-c27c-538a-9688-56d9ca17a7ec","is_active":true,"last_run":"2026-11-05T15:58:00Z","messages_sent":31402,"name":"Welcome series (7 days)","reentry":"ONCE","sequence":1,"status":"ACTIVE","step_count":9,"stop_on_conversion_id":"30eefab7-ff42-50cf-85bd-3a974b1ce548","stop_on_unsubscribe":true,"trigger_type":"SUBSCRIBED_TO_LIST","triggers":[{"config":{"label":"Newsletter"},"type":"SUBSCRIBED_TO_LIST"},{"config":{"label":"Website footer"},"type":"FORM_SUBMITTED"}],"updated_at":"2026-11-02T16:00:00Z"})).unwrap_or_default(), serde_json::from_value::<Workflow>(serde_json::json!({"active_enrolments":318,"completed_enrolments":2940,"created_at":"2026-06-18T16:00:00Z","description":"","entry_step_id":"686016a9-b02e-5415-8f48-28549c0ca454","goal_rate":0.11,"id":"1114e157-f1ec-5dda-9063-79abf0255ee3","is_active":true,"last_run":"2026-11-05T15:38:00Z","messages_sent":7110,"name":"Abandoned signup","reentry":"ONCE","sequence":1,"status":"ACTIVE","step_count":7,"stop_on_conversion_id":"ced25f3a-4a64-5e5a-a072-4ee34c3d0b5b","stop_on_unsubscribe":true,"trigger_type":"TAG_ADDED","triggers":[{"config":{"label":"trial"},"type":"TAG_ADDED"}],"updated_at":"2026-11-02T16:00:00Z"})).unwrap_or_default(), serde_json::from_value::<Workflow>(serde_json::json!({"active_enrolments":2077,"completed_enrolments":4120,"created_at":"2026-06-28T16:00:00Z","description":"","entry_step_id":"d8cdcb6a-61a1-563f-81fd-329e0ae430fd","goal_rate":0.06,"id":"b36c0bc7-7dae-56b4-af7f-9c1ef4673496","is_active":true,"last_run":"2026-11-05T15:18:00Z","messages_sent":4120,"name":"Re-engage · 90 days quiet","reentry":"MULTIPLE","sequence":1,"status":"ACTIVE","step_count":5,"stop_on_conversion_id":"ced25f3a-4a64-5e5a-a072-4ee34c3d0b5b","stop_on_unsubscribe":true,"trigger_type":"TIME_BASED","triggers":[{"config":{"label":"daily · segment Churn risk"},"type":"TIME_BASED"}],"updated_at":"2026-11-02T16:00:00Z"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 6, // TODO: Set from your business logic
    })
}
