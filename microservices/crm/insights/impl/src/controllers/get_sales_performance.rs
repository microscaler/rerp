// Implementation stub for handler 'get_sales_performance'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_insights_gen::handlers::get_sales_performance::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_insights_gen::handlers::types::DealStageRow;
#[allow(unused_imports)]
use rerp_crm_insights_gen::handlers::types::SalesPerformanceRow;

#[handler(GetSalesPerformanceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let pipeline_id = req.inner.pipeline_id;// let period = req.inner.period;// let owner_id = req.inner.owner_id;// let from = req.inner.from;// let to = req.inner.to;// let currency = req.inner.currency;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        currency: None, // TODO: Set from your business logic
        owners: vec![serde_json::from_value::<SalesPerformanceRow>(serde_json::json!({"lost_count":5,"name":"Alex Morgan","stages":[{"count":3,"name":"New","stage_id":"69eb8e38-4dda-5cd4-a12a-5ceefe3e043e","value":"14000.00"},{"count":4,"name":"Qualified","stage_id":"8d4abc9c-648c-5dbe-8668-b73ce7d9a71e","value":"52000.00"},{"count":5,"name":"Proposal","stage_id":"0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5","value":"88400.00"}],"user_id":"alex","won_count":12,"won_value":"96800.00"})).unwrap_or_default(), serde_json::from_value::<SalesPerformanceRow>(serde_json::json!({"lost_count":8,"name":"Sam Lee","stages":[{"count":4,"name":"New","stage_id":"69eb8e38-4dda-5cd4-a12a-5ceefe3e043e","value":"18000.00"},{"count":3,"name":"Qualified","stage_id":"8d4abc9c-648c-5dbe-8668-b73ce7d9a71e","value":"32400.00"},{"count":2,"name":"Proposal","stage_id":"0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5","value":"20100.00"}],"user_id":"sam","won_count":9,"won_value":"71200.00"})).unwrap_or_default(), serde_json::from_value::<SalesPerformanceRow>(serde_json::json!({"lost_count":3,"name":"Riya Kapoor","stages":[{"count":2,"name":"New","stage_id":"69eb8e38-4dda-5cd4-a12a-5ceefe3e043e","value":"9000.00"},{"count":1,"name":"Qualified","stage_id":"8d4abc9c-648c-5dbe-8668-b73ce7d9a71e","value":"12000.00"},{"count":0,"name":"Proposal","stage_id":"0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5","value":"0.00"}],"user_id":"riya","won_count":4,"won_value":"18400.00"})).unwrap_or_default()], // TODO: Set from your business logic
        period: None, // TODO: Set from your business logic
        pipeline_id: "d9a2022e-96ff-53a3-a94d-2b169424fd31".to_string(), // TODO: Set from your business logic
        stages: vec![serde_json::from_value::<DealStageRow>(serde_json::json!({"count":9,"name":"New","stage_id":"69eb8e38-4dda-5cd4-a12a-5ceefe3e043e","value":"41000.00"})).unwrap_or_default(), serde_json::from_value::<DealStageRow>(serde_json::json!({"count":8,"name":"Qualified","stage_id":"8d4abc9c-648c-5dbe-8668-b73ce7d9a71e","value":"96400.00"})).unwrap_or_default(), serde_json::from_value::<DealStageRow>(serde_json::json!({"count":7,"name":"Proposal","stage_id":"0bb5ee42-c41b-52ee-9c0a-99dbc4008ce5","value":"108500.00"})).unwrap_or_default()], // TODO: Set from your business logic
    })
}
