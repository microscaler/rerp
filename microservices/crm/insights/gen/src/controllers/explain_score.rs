// User-owned controller for handler 'explain_score'.

use crate::handlers::explain_score::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ExplainScoreController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //   "contact_name": "Priya Natarajan",
    //   "entity_id": "d7976b34-ac30-5901-8042-56e3d81193ab",
    //   "factors": [
    //     {
    //       "label": "Event: activated",
    //       "points": 25
    //     },
    //     {
    //       "label": "Plan is Business",
    //       "points": 20
    //     },
    //     {
    //       "label": "Visited /pricing",
    //       "points": 30
    //     }
    //   ],
    //   "score": 86,
    //   "score_id": "cef670be-5b12-52e7-8232-19a36fb64702"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "contact_id": "af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
  "contact_name": "Priya Natarajan",
  "entity_id": "d7976b34-ac30-5901-8042-56e3d81193ab",
  "factors": [
    {
      "label": "Event: activated",
      "points": 25
    },
    {
      "label": "Plan is Business",
      "points": 20
    },
    {
      "label": "Visited /pricing",
      "points": 30
    }
  ],
  "score": 86,
  "score_id": "cef670be-5b12-52e7-8232-19a36fb64702"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        band: Some("example".to_string()),
        contact_id: Some("af512c4f-0beb-50b6-bf61-7a9d8bd417b5".to_string()),
        contact_name: Some("Priya Natarajan".to_string()),
        entity_id: "d7976b34-ac30-5901-8042-56e3d81193ab".to_string(),
        entity_name: Some("example".to_string()),
        factors: vec![
            serde_json::from_str::<serde_json::Value>(
                r#"{"label":"Event: activated","points":25}"#,
            )
            .unwrap_or(serde_json::Value::Null),
            serde_json::from_str::<serde_json::Value>(
                r#"{"label":"Plan is Business","points":20}"#,
            )
            .unwrap_or(serde_json::Value::Null),
            serde_json::from_str::<serde_json::Value>(
                r#"{"label":"Visited /pricing","points":30}"#,
            )
            .unwrap_or(serde_json::Value::Null),
        ],
        score: 86,
        score_id: "cef670be-5b12-52e7-8232-19a36fb64702".to_string(),
    })
}
