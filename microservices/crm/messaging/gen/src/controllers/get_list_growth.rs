// User-owned controller for handler 'get_list_growth'.

use crate::handlers::get_list_growth::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetListGrowthController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "list_id": "03493116-74f7-52f2-8ad2-d3daeb50dfc8",
    //   "points": [
    //     {
    //       "date": "2026-11-25",
    //       "subscribed": 3,
    //       "total": 2,
    //       "unsubscribed": 3
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "list_id": "03493116-74f7-52f2-8ad2-d3daeb50dfc8",
  "points": [
    {
      "date": "2026-11-25",
      "subscribed": 3,
      "total": 2,
      "unsubscribed": 3
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
        list_id: "03493116-74f7-52f2-8ad2-d3daeb50dfc8".to_string(),
        points: vec![serde_json::from_str::<serde_json::Value>(
            r#"{"date":"2026-11-25","subscribed":3,"total":2,"unsubscribed":3}"#,
        )
        .unwrap_or(serde_json::Value::Null)],
    })
}
