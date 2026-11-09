// User-owned controller for handler 'update_score'.

use crate::handlers::update_score::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ScoreRule;

#[handler(UpdateScoreController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "counts": {
    //     "cold": 8920,
    //     "hot": 916,
    //     "warm": 2644
    //   },
    //   "created_at": "2026-07-28T16:00:00Z",
    //   "description": "email + site + lifecycle",
    //   "distribution": [
    //     42,
    //     30,
    //     22
    //   ],
    //   "entity": "CONTACT",
    //   "id": "cef670be-5b12-52e7-8232-19a36fb64702",
    //   "name": "Engagement",
    //   "rules": [
    //     {
    //       "decay_days": 30,
    //       "hits_30d": 23410,
    //       "id": "5876ef17-a412-5e28-bf53-4b86e2f712d0",
    //       "label": "Opened any email",
    //       "operator": "HAS",
    //       "points": 2,
    //       "subject": "engagement:opened:30",
    //       "value": ""
    //     },
    //     {
    //       "decay_days": 30,
    //       "hits_30d": 2804,
    //       "id": "e5c78629-9b55-50f5-b029-293852d13941",
    //       "label": "Clicked any email",
    //       "operator": "HAS",
    //       "points": 5,
    //       "subject": "engagement:clicked:30",
    //       "value": ""
    //     },
    //     {
    //       "decay_days": 14,
    //       "hits_30d": 4019,
    //       "id": "45d0c445-7fa8-5368-8120-32d4993f83b5",
    //       "label": "Visited /pricing",
    //       "operator": "HAS",
    //       "points": 15,
    //       "subject": "site:visited:/pricing",
    //       "value": ""
    //     }
    //   ],
    //   "status": "ACTIVE",
    //   "thresholds": {
    //     "hot": 70,
    //     "warm": 40
    //   }
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "counts": {
    "cold": 8920,
    "hot": 916,
    "warm": 2644
  },
  "created_at": "2026-07-28T16:00:00Z",
  "description": "email + site + lifecycle",
  "distribution": [
    42,
    30,
    22
  ],
  "entity": "CONTACT",
  "id": "cef670be-5b12-52e7-8232-19a36fb64702",
  "name": "Engagement",
  "rules": [
    {
      "decay_days": 30,
      "hits_30d": 23410,
      "id": "5876ef17-a412-5e28-bf53-4b86e2f712d0",
      "label": "Opened any email",
      "operator": "HAS",
      "points": 2,
      "subject": "engagement:opened:30",
      "value": ""
    },
    {
      "decay_days": 30,
      "hits_30d": 2804,
      "id": "e5c78629-9b55-50f5-b029-293852d13941",
      "label": "Clicked any email",
      "operator": "HAS",
      "points": 5,
      "subject": "engagement:clicked:30",
      "value": ""
    },
    {
      "decay_days": 14,
      "hits_30d": 4019,
      "id": "45d0c445-7fa8-5368-8120-32d4993f83b5",
      "label": "Visited /pricing",
      "operator": "HAS",
      "points": 15,
      "subject": "site:visited:/pricing",
      "value": ""
    }
  ],
  "status": "ACTIVE",
  "thresholds": {
    "hot": 70,
    "warm": 40
  }
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        counts: Some(serde_json::from_str::<serde_json::Value>(r#"{"cold":8920,"hot":916,"warm":2644}"#).unwrap_or(serde_json::Value::Null)),created_at: Some("2026-07-28T16:00:00Z".to_string()),description: Some("email + site + lifecycle".to_string()),distribution: Some(vec![42, 30, 22]),entity: "CONTACT".to_string(),id: "cef670be-5b12-52e7-8232-19a36fb64702".to_string(),name: "Engagement".to_string(),rules: Some(vec![serde_json::from_value::<ScoreRule>(serde_json::json!({"decay_days":30,"hits_30d":23410,"id":"5876ef17-a412-5e28-bf53-4b86e2f712d0","label":"Opened any email","operator":"HAS","points":2,"subject":"engagement:opened:30","value":""})).unwrap_or_default(), serde_json::from_value::<ScoreRule>(serde_json::json!({"decay_days":30,"hits_30d":2804,"id":"e5c78629-9b55-50f5-b029-293852d13941","label":"Clicked any email","operator":"HAS","points":5,"subject":"engagement:clicked:30","value":""})).unwrap_or_default(), serde_json::from_value::<ScoreRule>(serde_json::json!({"decay_days":14,"hits_30d":4019,"id":"45d0c445-7fa8-5368-8120-32d4993f83b5","label":"Visited /pricing","operator":"HAS","points":15,"subject":"site:visited:/pricing","value":""})).unwrap_or_default()]),status: "ACTIVE".to_string(),thresholds: serde_json::from_str::<serde_json::Value>(r#"{"hot":70,"warm":40}"#).unwrap_or(serde_json::Value::Null),updated_at: Some("example".to_string()),
    })
}
