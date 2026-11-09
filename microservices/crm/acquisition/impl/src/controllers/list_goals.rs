// Implementation stub for handler 'list_goals'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_acquisition_gen::handlers::list_goals::{Request, Response};

#[allow(
    unused_imports,
    reason = "generated stub imports every schema type the handler may need"
)]
use rerp_crm_acquisition_gen::handlers::types::Goal;

#[handler(ListGoalsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let page = req.inner.page;// let limit = req.inner.limit;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<Goal>(serde_json::json!({"conversion_id":"30eefab7-ff42-50cf-85bd-3a974b1ce548","conversion_name":"Trial signup","created_at":"2026-09-26T16:00:00Z","current":"4106.00","drivers":"Launch waitlist form 41% · google/cpc 22% · newsletter 14%","id":"8f4f6648-aa8e-5fb6-9207-6551da9714df","name":"Q4 trial signups","note":"68% with 55% of the quarter gone","period":"Oct–Dec 2026","status":"ON_TRACK","target":"6000.00","unit":"COUNT"})).unwrap_or_default(), serde_json::from_value::<Goal>(serde_json::json!({"conversion_id":"473f4a5c-dcac-59e5-b889-2eb270d9db5d","conversion_name":"Activated","created_at":"2026-09-26T16:00:00Z","current":"61.00","drivers":"Welcome series · Abandoned signup","id":"047a18d8-3dbe-5b48-8733-4d69800e8319","name":"Activation rate","note":"welcome series goal rate 23%","period":"Rolling 30 d","status":"BEHIND","target":"70.00","unit":"COUNT"})).unwrap_or_default(), serde_json::from_value::<Goal>(serde_json::json!({"conversion_id":"8e13bc76-1bf6-599a-8d70-42b478f2888a","conversion_name":"Deal won","created_at":"2026-09-26T16:00:00Z","current":"118000.00","drivers":"Hot lead handoff €21.6k · BF early access (scheduled)","id":"2bc436a2-cd67-5245-90eb-af548d2f64ef","name":"New revenue · November","note":"ahead of pace","period":"Nov 2026","status":"AHEAD","target":"200000.00","unit":"COUNT"})).unwrap_or_default()], // TODO: Set from your business logic
        limit: 25, // TODO: Set from your business logic
        page: 1, // TODO: Set from your business logic
        total: 3, // TODO: Set from your business logic
    })
}
