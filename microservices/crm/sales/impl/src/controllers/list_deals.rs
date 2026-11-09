// Implementation stub for handler 'list_deals'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_sales_gen::handlers::list_deals::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_sales_gen::handlers::types::Deal;

#[handler(ListDealsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let search = req.inner.search;// let pipeline_id = req.inner.pipeline_id;// let stage_id = req.inner.stage_id;// let status = req.inner.status;// let owner_id = req.inner.owner_id;// let contact_id = req.inner.contact_id;// let organisation_id = req.inner.organisation_id;// let tag_id = req.inner.tag_id;// let stalled = req.inner.stalled;// let expected_close_from = req.inner.expected_close_from;// let expected_close_to = req.inner.expected_close_to;// let value_min = req.inner.value_min;// let value_max = req.inner.value_max;// let sort = req.inner.sort;// let dir = req.inner.dir;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Deal>(serde_json::json!({"contacts":[{"contact_id":"af512c4f-0beb-50b6-bf61-7a9d8bd417b5","name":"Priya Natarajan","role":"Primary contact"}],"created_at":"2026-10-13T16:00:00Z","currency":"EUR","custom_fields":{"seats":10,"term_months":"12"},"days_in_stage":3,"expected_close_date":"2026-11-25","id":"63a6661f-5860-50ed-8697-f28d29a3917f","last_activity_at":"2026-11-02T16:00:00Z","next_task_at":"2026-11-06T16:00:00Z","organisation_id":"d7a2e2b8-61b1-5687-8aee-0ead8943ba3b","organisation_name":"Northwind Traders","owner_id":"alex","owner_name":"Alex Morgan","pipeline_id":"d9a2022e-96ff-53a3-a94d-2b169424fd31","probability":50,"source":"customer expansion","stage_id":"0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5","stage_name":"Proposal","status":"OPEN","title":"Northwind · 48 → 120 seats","updated_at":"2026-11-02T16:00:00Z","value":"38400.00","weighted_value":"19200.00"})).unwrap_or_default(), serde_json::from_value::<Deal>(serde_json::json!({"contacts":[{"contact_id":"7ed65e2d-0bd7-5e4e-9071-f4dac3456426","name":"Tom Okafor","role":"Primary contact"}],"created_at":"2026-10-14T16:00:00Z","currency":"EUR","custom_fields":{"seats":14,"term_months":"12"},"days_in_stage":2,"expected_close_date":"2026-12-01","id":"48d65ced-2142-5626-af8e-c006f3c081b0","last_activity_at":"2026-11-03T16:00:00Z","next_task_at":"2026-11-07T16:00:00Z","organisation_id":"576a7630-a5b2-566c-ac00-caca59662cb0","organisation_name":"Fabrikam Inc","owner_id":"sam","owner_name":"Sam Lee","pipeline_id":"d9a2022e-96ff-53a3-a94d-2b169424fd31","probability":10,"source":"form: free trial","stage_id":"69eb8e38-4dda-5cd4-a12a-5ceefe3e043e","stage_name":"New","status":"OPEN","title":"Fabrikam · trial → team","updated_at":"2026-11-03T16:00:00Z","value":"4800.00","weighted_value":"480.00"})).unwrap_or_default(), serde_json::from_value::<Deal>(serde_json::json!({"contacts":[{"contact_id":"f25ace60-6582-5cc7-9226-f9367248776c","name":"Lena Fischer","role":"Primary contact"}],"created_at":"2026-10-08T16:00:00Z","currency":"EUR","custom_fields":{"seats":18,"term_months":"12"},"days_in_stage":8,"expected_close_date":"2026-12-07","id":"9c1e9e9d-0f28-58fc-bef5-2374e4abf12c","last_activity_at":"2026-10-28T16:00:00Z","next_task_at":"2026-11-08T16:00:00Z","organisation_id":"da09c739-6e09-5587-a658-88f6b9eea0bd","organisation_name":"Contoso Ltd","owner_id":"riya","owner_name":"Riya Kapoor","pipeline_id":"d9a2022e-96ff-53a3-a94d-2b169424fd31","probability":50,"source":"form: contact sales","stage_id":"0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5","stage_name":"Proposal","status":"OPEN","title":"Contoso · Business plan","updated_at":"2026-10-28T16:00:00Z","value":"24000.00","weighted_value":"12000.00"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 12, // TODO: Set from your business logic
    })
}
