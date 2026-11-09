// User-owned controller for handler 'import_workflow_recipe'.

use crate::handlers::import_workflow_recipe::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::RecipeParameter;
#[allow(unused_imports)]
use crate::handlers::types::TriggerType;

#[handler(ImportWorkflowRecipeController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "category": "WELCOME",
    //   "definition": {},
    //   "description": "Example",
    //   "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
    //   "industry": "Industry",
    //   "name": "Northwind Traders",
    //   "needs_templates": 3,
    //   "parameters": [
    //     {
    //       "key": "snippet.signature",
    //       "kind": "LIST",
    //       "label": "Primary"
    //     }
    //   ],
    //   "source_url": "https://northwindtraders.example/pricing",
    //   "step_count": 3,
    //   "trigger_type": "STAGE_CHANGE",
    //   "trigger_types": [
    //     "STAGE_CHANGE"
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "category": "WELCOME",
  "definition": {},
  "description": "Example",
  "id": "c9e18116-9fea-5e6e-ac32-52580d2450b8",
  "industry": "Industry",
  "name": "Northwind Traders",
  "needs_templates": 3,
  "parameters": [
    {
      "key": "snippet.signature",
      "kind": "LIST",
      "label": "Primary"
    }
  ],
  "source_url": "https://northwindtraders.example/pricing",
  "step_count": 3,
  "trigger_type": "STAGE_CHANGE",
  "trigger_types": [
    "STAGE_CHANGE"
  ]
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        category: "WELCOME".to_string(),
        definition: Some(
            serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null),
        ),
        description: Some("Example".to_string()),
        id: "c9e18116-9fea-5e6e-ac32-52580d2450b8".to_string(),
        industry: Some("Industry".to_string()),
        name: "Northwind Traders".to_string(),
        needs_templates: Some(3),
        parameters: Some(vec![serde_json::from_value::<RecipeParameter>(
            serde_json::json!({"key":"snippet.signature","kind":"LIST","label":"Primary"}),
        )
        .unwrap_or_default()]),
        source_url: Some("https://northwindtraders.example/pricing".to_string()),
        step_count: Some(3),
        trigger_type: Some(
            serde_json::from_value::<TriggerType>(serde_json::Value::String(
                "STAGE_CHANGE".to_string(),
            ))
            .unwrap_or_default(),
        ),
        trigger_types: Some(vec![serde_json::from_value::<TriggerType>(
            serde_json::Value::String("STAGE_CHANGE".to_string()),
        )
        .unwrap_or_default()]),
    })
}
