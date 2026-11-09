// User-owned controller for handler 'activate_score'.

use crate::handlers::activate_score::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ScoreRule;

#[handler(ActivateScoreController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "counts": {
    //     "cold": 3,
    //     "hot": 3,
    //     "warm": 3
    //   },
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "description": "Example",
    //   "distribution": [
    //     3
    //   ],
    //   "entity": "CONTACT",
    //   "id": "aa70dbe4-37c6-567e-b827-49d9e6bcd18c",
    //   "name": "Northwind Traders",
    //   "rules": [
    //     {
    //       "operator": "Operator",
    //       "points": 3,
    //       "subject": "October product update"
    //     }
    //   ],
    //   "status": "DRAFT",
    //   "thresholds": {
    //     "hot": 3,
    //     "warm": 3
    //   },
    //   "updated_at": "2026-11-05T16:00:00Z"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "counts": {
    "cold": 3,
    "hot": 3,
    "warm": 3
  },
  "created_at": "2026-11-05T16:00:00Z",
  "description": "Example",
  "distribution": [
    3
  ],
  "entity": "CONTACT",
  "id": "aa70dbe4-37c6-567e-b827-49d9e6bcd18c",
  "name": "Northwind Traders",
  "rules": [
    {
      "operator": "Operator",
      "points": 3,
      "subject": "October product update"
    }
  ],
  "status": "DRAFT",
  "thresholds": {
    "hot": 3,
    "warm": 3
  },
  "updated_at": "2026-11-05T16:00:00Z"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        counts: Some(serde_json::from_str::<serde_json::Value>(r#"{"cold":3,"hot":3,"warm":3}"#).unwrap_or(serde_json::Value::Null)),created_at: Some("2026-11-05T16:00:00Z".to_string()),description: Some("Example".to_string()),distribution: Some(vec![3]),entity: "CONTACT".to_string(),id: "aa70dbe4-37c6-567e-b827-49d9e6bcd18c".to_string(),name: "Northwind Traders".to_string(),rules: Some(vec![serde_json::from_value::<ScoreRule>(serde_json::json!({"operator":"Operator","points":3,"subject":"October product update"})).unwrap_or_default()]),status: "DRAFT".to_string(),thresholds: serde_json::from_str::<serde_json::Value>(r#"{"hot":3,"warm":3}"#).unwrap_or(serde_json::Value::Null),updated_at: Some("2026-11-05T16:00:00Z".to_string()),
    })
}
