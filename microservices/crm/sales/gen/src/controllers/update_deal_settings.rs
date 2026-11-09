// User-owned controller for handler 'update_deal_settings'.

use crate::handlers::update_deal_settings::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateDealSettingsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "default_currency": "Default currency",
    //   "default_pipeline_id": "7a9c6742-b6a4-54c0-bae3-e0578533dc6e",
    //   "require_expected_close_from_stage_id": "6e025ea4-2f9b-5460-b23f-834aa8efce87",
    //   "win_probability": {
    //     "key": {}
    //   }
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "default_currency": "Default currency",
  "default_pipeline_id": "7a9c6742-b6a4-54c0-bae3-e0578533dc6e",
  "require_expected_close_from_stage_id": "6e025ea4-2f9b-5460-b23f-834aa8efce87",
  "win_probability": {
    "key": {}
  }
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        default_currency: Some("Default currency".to_string()),
        default_pipeline_id: Some("7a9c6742-b6a4-54c0-bae3-e0578533dc6e".to_string()),
        require_expected_close_from_stage_id: Some(
            "6e025ea4-2f9b-5460-b23f-834aa8efce87".to_string(),
        ),
        win_probability: Some(
            serde_json::from_str::<serde_json::Value>(r#"{"key":{}}"#)
                .unwrap_or(serde_json::Value::Null),
        ),
    })
}
