// User-owned controller for handler 'get_widget_data'.

use crate::handlers::get_widget_data::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::NameCount;
#[allow(unused_imports)]
use crate::handlers::types::NamedSeries;

#[handler(GetWidgetDataController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "refreshed_at": "2026-11-05T16:00:00Z",
    //   "rows": [
    //     {}
    //   ],
    //   "series": [
    //     {
    //       "key": "snippet.signature",
    //       "points": [
    //         {
    //           "date": "2026-11-25",
    //           "value": 12.5
    //         }
    //       ]
    //     }
    //   ],
    //   "values": [
    //     {
    //       "count": 3,
    //       "name": "Northwind Traders"
    //     }
    //   ],
    //   "widget_id": "1590c639-a479-5154-9846-191756192e74"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "refreshed_at": "2026-11-05T16:00:00Z",
  "rows": [
    {}
  ],
  "series": [
    {
      "key": "snippet.signature",
      "points": [
        {
          "date": "2026-11-25",
          "value": 12.5
        }
      ]
    }
  ],
  "values": [
    {
      "count": 3,
      "name": "Northwind Traders"
    }
  ],
  "widget_id": "1590c639-a479-5154-9846-191756192e74"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        refreshed_at: "2026-11-05T16:00:00Z".to_string(),rows: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{}"#).unwrap_or(serde_json::Value::Null)]),series: Some(vec![serde_json::from_value::<NamedSeries>(serde_json::json!({"key":"snippet.signature","points":[{"date":"2026-11-25","value":12.5}]})).unwrap_or_default()]),values: Some(vec![serde_json::from_value::<NameCount>(serde_json::json!({"count":3,"name":"Northwind Traders"})).unwrap_or_default()]),widget_id: "1590c639-a479-5154-9846-191756192e74".to_string(),
    })
}
