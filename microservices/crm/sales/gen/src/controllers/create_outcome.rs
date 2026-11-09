// User-owned controller for handler 'create_outcome'.

use crate::handlers::create_outcome::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Sentiment;

#[handler(CreateOutcomeController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "id": "0a7437f6-0e3e-51ae-a9fd-4a44b900f91e",
    //   "label": "Reached",
    //   "sentiment": "POSITIVE",
    //   "task_type_ids": []
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "id": "0a7437f6-0e3e-51ae-a9fd-4a44b900f91e",
  "label": "Reached",
  "sentiment": "POSITIVE",
  "task_type_ids": []
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        count_30d: Some(42),
        id: "0a7437f6-0e3e-51ae-a9fd-4a44b900f91e".to_string(),
        label: "Reached".to_string(),
        sentiment: serde_json::from_value::<Sentiment>(serde_json::Value::String(
            "POSITIVE".to_string(),
        ))
        .unwrap_or_default(),
        task_type_ids: Some(vec![]),
        used_by_type_ids: Some(vec![]),
    })
}
