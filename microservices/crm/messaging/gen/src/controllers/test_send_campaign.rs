// User-owned controller for handler 'test_send_campaign'.

use crate::handlers::test_send_campaign::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(TestSendCampaignController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "sent_to": [
    //     "priya.natarajan@northwindtraders.example"
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "sent_to": [
    "priya.natarajan@northwindtraders.example"
  ]
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        message_ids: Some(vec![]),
        sent_to: vec!["priya.natarajan@northwindtraders.example".to_string()],
    })
}
