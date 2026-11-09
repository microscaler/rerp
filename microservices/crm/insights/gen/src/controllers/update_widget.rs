// User-owned controller for handler 'update_widget'.

use crate::handlers::update_widget::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ReportSchedule;
#[allow(unused_imports)]
use crate::handlers::types::WidgetQuery;

#[handler(UpdateWidgetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "id": "aa70dbe4-37c6-567e-b827-49d9e6bcd18c",
    //   "kind": "NUMBER",
    //   "position": 3,
    //   "prompt": "Prompt",
    //   "query": {
    //     "dimension": "Dimension",
    //     "end_date": "2026-11-25",
    //     "filters": {},
    //     "granularity": "DAY",
    //     "limit": 25,
    //     "metric": "Metric",
    //     "range": "LAST_7_DAYS",
    //     "start_date": "2026-11-25"
    //   },
    //   "refreshed_at": "2026-11-05T16:00:00Z",
    //   "schedule": {
    //     "frequency": "Frequency",
    //     "id": "aa70dbe4-37c6-567e-b827-49d9e6bcd18c",
    //     "is_active": true,
    //     "last_sent": "2026-11-05T16:00:00Z",
    //     "recipient_ids": [
    //       "74ac0043-87c0-5717-854d-594f606bbda7"
    //     ],
    //     "report_id": "00daadeb-6175-5df5-b365-7a76ec2eaa99"
    //   },
    //   "title": "Northwind · 48 → 120 seats",
    //   "width": 1
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "id": "aa70dbe4-37c6-567e-b827-49d9e6bcd18c",
  "kind": "NUMBER",
  "position": 3,
  "prompt": "Prompt",
  "query": {
    "dimension": "Dimension",
    "end_date": "2026-11-25",
    "filters": {},
    "granularity": "DAY",
    "limit": 25,
    "metric": "Metric",
    "range": "LAST_7_DAYS",
    "start_date": "2026-11-25"
  },
  "refreshed_at": "2026-11-05T16:00:00Z",
  "schedule": {
    "frequency": "Frequency",
    "id": "aa70dbe4-37c6-567e-b827-49d9e6bcd18c",
    "is_active": true,
    "last_sent": "2026-11-05T16:00:00Z",
    "recipient_ids": [
      "74ac0043-87c0-5717-854d-594f606bbda7"
    ],
    "report_id": "00daadeb-6175-5df5-b365-7a76ec2eaa99"
  },
  "title": "Northwind · 48 → 120 seats",
  "width": 1
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        id: "aa70dbe4-37c6-567e-b827-49d9e6bcd18c".to_string(),kind: "NUMBER".to_string(),position: Some(3),prompt: Some("Prompt".to_string()),query: serde_json::from_value::<WidgetQuery>(serde_json::json!({"dimension":"Dimension","end_date":"2026-11-25","filters":{},"granularity":"DAY","limit":25,"metric":"Metric","range":"LAST_7_DAYS","start_date":"2026-11-25"})).unwrap_or_default(),refreshed_at: Some("2026-11-05T16:00:00Z".to_string()),schedule: Some(serde_json::from_value::<ReportSchedule>(serde_json::json!({"frequency":"Frequency","id":"aa70dbe4-37c6-567e-b827-49d9e6bcd18c","is_active":true,"last_sent":"2026-11-05T16:00:00Z","recipient_ids":["74ac0043-87c0-5717-854d-594f606bbda7"],"report_id":"00daadeb-6175-5df5-b365-7a76ec2eaa99"})).unwrap_or_default()),title: "Northwind · 48 → 120 seats".to_string(),width: Some(1),
    })
}
