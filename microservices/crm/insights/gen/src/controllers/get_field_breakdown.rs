// User-owned controller for handler 'get_field_breakdown'.

use crate::handlers::get_field_breakdown::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::NameCount;

#[handler(GetFieldBreakdownController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "blank": 4204,
    //   "entity": "CONTACT",
    //   "field_id": "0bbefc02-42a4-5469-bb31-d1fba48fc38a",
    //   "field_label": "Plan",
    //   "values": [
    //     {
    //       "count": 7902,
    //       "name": "Free"
    //     },
    //     {
    //       "count": 3114,
    //       "name": "Team"
    //     },
    //     {
    //       "count": 2210,
    //       "name": "Business"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "blank": 4204,
  "entity": "CONTACT",
  "field_id": "0bbefc02-42a4-5469-bb31-d1fba48fc38a",
  "field_label": "Plan",
  "values": [
    {
      "count": 7902,
      "name": "Free"
    },
    {
      "count": 3114,
      "name": "Team"
    },
    {
      "count": 2210,
      "name": "Business"
    }
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
        blank: Some(4204),
        entity: "CONTACT".to_string(),
        field_id: "0bbefc02-42a4-5469-bb31-d1fba48fc38a".to_string(),
        field_label: Some("Plan".to_string()),
        values: vec![
            serde_json::from_value::<NameCount>(serde_json::json!({"count":7902,"name":"Free"}))
                .unwrap_or_default(),
            serde_json::from_value::<NameCount>(serde_json::json!({"count":3114,"name":"Team"}))
                .unwrap_or_default(),
            serde_json::from_value::<NameCount>(
                serde_json::json!({"count":2210,"name":"Business"}),
            )
            .unwrap_or_default(),
        ],
    })
}
