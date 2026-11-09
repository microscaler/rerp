// User-owned controller for handler 'list_senders'.

use crate::handlers::list_senders::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Sender;

#[handler(ListSendersController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "created_at": "2026-11-05T16:00:00Z",
    //       "domain": "northwindtraders.example",
    //       "from_address": "priya.natarajan@northwindtraders.example",
    //       "from_name": "From name",
    //       "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
    //       "status": "PENDING"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "created_at": "2026-11-05T16:00:00Z",
      "domain": "northwindtraders.example",
      "from_address": "priya.natarajan@northwindtraders.example",
      "from_name": "From name",
      "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
      "status": "PENDING"
    }
  ]
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response(vec![serde_json::from_value::<Sender>(serde_json::json!({"created_at":"2026-11-05T16:00:00Z","domain":"northwindtraders.example","from_address":"priya.natarajan@northwindtraders.example","from_name":"From name","id":"3eb009b7-a99d-5a4c-956a-99e11d8dbb76","status":"PENDING"})).unwrap_or_default()]))
}
