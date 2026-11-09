// User-owned controller for handler 'get_pipeline'.

use crate::handlers::get_pipeline::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetPipelineController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "auto_assign": "ROUND_ROBIN",
    //   "created_at": "2026-03-30T16:00:00Z",
    //   "currency": "EUR",
    //   "id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
    //   "name": "Sales",
    //   "open_deals": 38,
    //   "open_value": "412300.00",
    //   "visibility": "ALL"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "auto_assign": "ROUND_ROBIN",
  "created_at": "2026-03-30T16:00:00Z",
  "currency": "EUR",
  "id": "d9a2022e-96ff-53a3-a94d-2b169424fd31",
  "name": "Sales",
  "open_deals": 38,
  "open_value": "412300.00",
  "visibility": "ALL"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        auto_assign: Some("ROUND_ROBIN".to_string()),
        auto_assign_user_ids: Some(vec![]),
        created_at: "2026-03-30T16:00:00Z".to_string(),
        currency: "EUR".to_string(),
        forecast_mode: Some("example".to_string()),
        group_ids: Some(vec![]),
        id: "d9a2022e-96ff-53a3-a94d-2b169424fd31".to_string(),
        name: "Sales".to_string(),
        open_deals: Some(38),
        open_value: Some("412300.00".to_string()),
        sequence: Some(42),
        updated_at: Some("example".to_string()),
        visibility: Some("ALL".to_string()),
    })
}
