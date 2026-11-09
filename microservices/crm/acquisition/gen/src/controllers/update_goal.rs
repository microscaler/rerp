// User-owned controller for handler 'update_goal'.

use crate::handlers::update_goal::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateGoalController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "conversion_id": "7c5ad264-f0bb-54cd-a7f8-a78df094f9bb",
    //   "conversion_name": "Conversion name",
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "current": "1250.00",
    //   "drivers": "Drivers",
    //   "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
    //   "name": "Northwind Traders",
    //   "note": "Note",
    //   "period": "Period",
    //   "period_end": "2026-11-25",
    //   "period_start": "2026-11-25",
    //   "status": "ON_TRACK",
    //   "target": "1250.00",
    //   "unit": "COUNT"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "conversion_id": "7c5ad264-f0bb-54cd-a7f8-a78df094f9bb",
  "conversion_name": "Conversion name",
  "created_at": "2026-11-05T16:00:00Z",
  "current": "1250.00",
  "drivers": "Drivers",
  "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
  "name": "Northwind Traders",
  "note": "Note",
  "period": "Period",
  "period_end": "2026-11-25",
  "period_start": "2026-11-25",
  "status": "ON_TRACK",
  "target": "1250.00",
  "unit": "COUNT"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        conversion_id: "7c5ad264-f0bb-54cd-a7f8-a78df094f9bb".to_string(),
        conversion_name: Some("Conversion name".to_string()),
        created_at: Some("2026-11-05T16:00:00Z".to_string()),
        current: Some("1250.00".to_string()),
        drivers: Some("Drivers".to_string()),
        id: "e67555dd-33f4-5fee-9da3-a019895219b1".to_string(),
        name: "Northwind Traders".to_string(),
        note: Some("Note".to_string()),
        period: Some("Period".to_string()),
        period_end: Some("2026-11-25".to_string()),
        period_start: Some("2026-11-25".to_string()),
        status: Some("ON_TRACK".to_string()),
        target: "1250.00".to_string(),
        unit: Some("COUNT".to_string()),
    })
}
