// Implementation stub for handler 'list_leads'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_sales_gen::handlers::list_leads::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_sales_gen::handlers::types::Lead;

#[handler(ListLeadsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let search = req.inner.search;// let type = req.inner.type;// let stage_id = req.inner.stage_id;// let user_id = req.inner.user_id;// let team_id = req.inner.team_id;// let probability_gt = req.inner.probability_gt;// let active = req.inner.active;// let won_status = req.inner.won_status;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Lead>(serde_json::json!({"active":true,"attribution":{"channel":"customer expansion","first_touch_at":"2026-10-13T16:00:00Z","landing_path":"/pricing"},"contact_name":"Priya Natarajan","create_date":"2026-10-13T16:00:00Z","date_last_stage_update":"2026-11-02T16:00:00Z","description":"Source: customer expansion","email_from":"priya.natarajan@northwindtraders.example","expected_revenue":"38400.00","form":"Form","id":"63a6661f-5860-50ed-8697-f28d29a3917f","name":"Northwind · 48 → 120 seats","organisation_id":"d7a2e2b8-61b1-5687-8aee-0ead8943ba3b","organisation_name":"Northwind Traders","phone":"+49 30 1000000","priority":"NORMAL","probability":50,"stage_id":"0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5","stage_name":"Proposal","stage_probability":50,"tag_ids":[],"team_id":"00000000-0000-0000-0000-000000000401","type":"LEAD","won_status":"PENDING","write_date":"2026-11-02T16:00:00Z"})).unwrap_or_default(), serde_json::from_value::<Lead>(serde_json::json!({"active":true,"attribution":{"channel":"form","first_touch_at":"2026-10-14T16:00:00Z","landing_path":"/pricing"},"contact_name":"Tom Okafor","create_date":"2026-10-14T16:00:00Z","date_last_stage_update":"2026-11-03T16:00:00Z","description":"Source: form: free trial","email_from":"tom.okafor@fabrikam.example","expected_revenue":"4800.00","form":"contact","id":"48d65ced-2142-5626-af8e-c006f3c081b0","name":"Fabrikam · trial → team","organisation_id":"576a7630-a5b2-566c-ac00-caca59662cb0","organisation_name":"Fabrikam Inc","phone":"+49 30 1007919","priority":"HIGH","probability":10,"stage_id":"69eb8e38-4dda-5cd4-a12a-5ceefe3e043e","stage_name":"New","stage_probability":10,"tag_ids":[],"team_id":"00000000-0000-0000-0000-000000000401","type":"LEAD","won_status":"PENDING","write_date":"2026-11-03T16:00:00Z"})).unwrap_or_default(), serde_json::from_value::<Lead>(serde_json::json!({"active":true,"attribution":{"channel":"form","first_touch_at":"2026-10-08T16:00:00Z","landing_path":"/pricing"},"contact_name":"Lena Fischer","create_date":"2026-10-08T16:00:00Z","date_last_stage_update":"2026-10-28T16:00:00Z","description":"Source: form: contact sales","email_from":"lena.fischer@contoso.example","expected_revenue":"24000.00","form":"contact","id":"9c1e9e9d-0f28-58fc-bef5-2374e4abf12c","name":"Contoso · Business plan","organisation_id":"da09c739-6e09-5587-a658-88f6b9eea0bd","organisation_name":"Contoso Ltd","phone":"+49 30 1015838","priority":"LOW","probability":50,"stage_id":"0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5","stage_name":"Proposal","stage_probability":50,"tag_ids":[],"team_id":"00000000-0000-0000-0000-000000000401","type":"LEAD","won_status":"PENDING","write_date":"2026-10-28T16:00:00Z"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 12, // TODO: Set from your business logic
    })
}
