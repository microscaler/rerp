// Implementation stub for handler 'get_tag_trends'
// This file is a starting point for your implementation.
// After implementing business logic, add this line at the top to protect from --force regen:
//   // BRRTRouter: user-owned
// To create missing stubs only: brrtrouter-gen generate-stubs
// To patch signature/Response on protected stubs: brrtrouter-gen generate-stubs --sync

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_insights_gen::handlers::get_tag_trends::{Request, Response};

#[allow(unused_imports)]
use rerp_crm_insights_gen::handlers::types::NameCount;
#[allow(unused_imports)]
use rerp_crm_insights_gen::handlers::types::NamedSeries;
#[allow(unused_imports)]
use rerp_crm_insights_gen::handlers::types::TagTrendRow;

#[handler(GetTagTrendsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // TODO: Implement your business logic here
    //
    // Example: Access request data
    // let x_workspace_id = req.inner.x_workspace_id;// let tag_ids = req.inner.tag_ids;// let granularity = req.inner.granularity;// let from = req.inner.from;// let to = req.inner.to;
    //
    // Example: Database query, validation, etc.
    // let result = your_service.process(&req.inner)?;
    //
    // Example: Return response

    HttpJson::ok(Response {
        avg_per_day: 74.8, // TODO: Set from your business logic
        most_used_in_range: None, // TODO: Set from your business logic
        most_used_overall: None, // TODO: Set from your business logic
        series: vec![serde_json::from_value::<NamedSeries>(serde_json::json!({"key":"lead","label":"lead","points":[{"date":"2026-10-07","value":30},{"date":"2026-10-08","value":37},{"date":"2026-10-09","value":33}]})).unwrap_or_default(), serde_json::from_value::<NamedSeries>(serde_json::json!({"key":"trial","label":"trial","points":[{"date":"2026-10-07","value":14},{"date":"2026-10-08","value":21},{"date":"2026-10-09","value":17}]})).unwrap_or_default(), serde_json::from_value::<NamedSeries>(serde_json::json!({"key":"customer","label":"customer","points":[{"date":"2026-10-07","value":2},{"date":"2026-10-08","value":9},{"date":"2026-10-09","value":5}]})).unwrap_or_default()], // TODO: Set from your business logic
        tags: vec![serde_json::from_value::<TagTrendRow>(serde_json::json!({"applied_in_range":1104,"avg_per_day":35.6,"first_applied_at":"2026-01-02T00:00:00Z","last_applied_at":"2026-11-05T16:00:00Z","name":"lead","tag_id":"9fcf4e9e-e339-5eef-8d3d-9086b582243e","total_contacts":6114})).unwrap_or_default(), serde_json::from_value::<TagTrendRow>(serde_json::json!({"applied_in_range":612,"avg_per_day":19.7,"first_applied_at":"2026-02-14T00:00:00Z","last_applied_at":"2026-11-05T16:00:00Z","name":"trial","tag_id":"9ea75ff5-0642-5599-9bf6-34f9bbcefd89","total_contacts":1412})).unwrap_or_default(), serde_json::from_value::<TagTrendRow>(serde_json::json!({"applied_in_range":301,"avg_per_day":9.7,"first_applied_at":"2026-03-03T00:00:00Z","last_applied_at":"2026-11-05T16:00:00Z","name":"pricing-page","tag_id":"83c2db54-32d0-50e6-a226-f6487560ac02","total_contacts":902})).unwrap_or_default()], // TODO: Set from your business logic
        tags_applied: 2318, // TODO: Set from your business logic
    })
}
