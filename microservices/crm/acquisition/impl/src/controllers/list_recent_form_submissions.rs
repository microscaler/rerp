// Implementation stub for handler 'list_recent_form_submissions'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_acquisition_gen::handlers::list_recent_form_submissions::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_acquisition_gen::handlers::types::FormSubmission;

#[handler(ListRecentFormSubmissionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;// let form_id = req.inner.form_id;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<FormSubmission>(serde_json::json!({"at":"2026-11-05T15:40:00Z","contact_id":"5de76105-f8c3-5357-af13-c5ce28753ed0","contact_name":"Jonas Weber","email":"jonas.weber@proseware.example","form_id":"454cdf11-2c7b-505e-8864-df16d18ca8cc","form_name":"Website footer","id":"33862d04-0db3-5f10-8b9e-da432aa545ec","submitted_at":"2026-11-05T16:00:00Z","utm_campaign":"q4-webinars","utm_medium":null,"utm_source":"linkedin","values":{"key":"Value"}})).unwrap_or_default(), serde_json::from_value::<FormSubmission>(serde_json::json!({"at":"2026-11-02T16:00:00Z","contact_id":"f25ace60-6582-5cc7-9226-f9367248776c","contact_name":"Lena Fischer","email":"lena.fischer@contoso.example","form_id":"d614ffd8-570e-5b8f-b84c-75324e12322e","form_name":"Launch waitlist","id":"77a87e6a-0d20-5e66-ac8b-b889d993c003","submitted_at":"2026-11-05T16:00:00Z","utm_campaign":"Utm campaign","utm_medium":"cpc","utm_source":"google","values":{"key":"Value"}})).unwrap_or_default(), serde_json::from_value::<FormSubmission>(serde_json::json!({"at":"2026-10-28T16:00:00Z","contact_id":"7ed65e2d-0bd7-5e4e-9071-f4dac3456426","contact_name":"Tom Okafor","email":"tom.okafor@fabrikam.example","form_id":"997e0128-1f73-5fb4-b47e-b72197474a68","form_name":"Free trial","id":"b14461b9-7786-5357-9035-86f330642d09","submitted_at":"2026-11-05T16:00:00Z","utm_campaign":"Utm campaign","utm_medium":null,"utm_source":"google","values":{"key":"Value"}})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 6, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 4, // TODO: Set from your business logic
    })
}
