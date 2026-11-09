// User-owned controller for handler 'test_send_template'.

use crate::handlers::test_send_template::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Channel;
#[allow(unused_imports)]
use crate::handlers::types::MessageEvent;
#[allow(unused_imports)]
use crate::handlers::types::Provider;
#[allow(unused_imports)]
use crate::handlers::types::SendStatus;

#[handler(TestSendTemplateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "channel": "EMAIL",
    //   "contact_id": "c891a339-996d-56b9-a022-244c570f48c9",
    //   "created_at": "2026-11-05T16:00:00Z",
    //   "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
    //   "queued": 1,
    //   "status": "QUEUED"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "channel": "EMAIL",
  "contact_id": "c891a339-996d-56b9-a022-244c570f48c9",
  "created_at": "2026-11-05T16:00:00Z",
  "id": "3eb009b7-a99d-5a4c-956a-99e11d8dbb76",
  "queued": 1,
  "status": "QUEUED"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        campaign_id: Some("example".to_string()),
        campaign_name: Some("example".to_string()),
        channel: serde_json::from_value::<Channel>(serde_json::Value::String("EMAIL".to_string()))
            .unwrap_or_default(),
        clicked_at: Some("example".to_string()),
        clicked_link: Some("example".to_string()),
        contact_id: "c891a339-996d-56b9-a022-244c570f48c9".to_string(),
        contact_name: Some("example".to_string()),
        created_at: "2026-11-05T16:00:00Z".to_string(),
        enrolment_id: Some("example".to_string()),
        error: Some("example".to_string()),
        events: Some(vec![]),
        id: "3eb009b7-a99d-5a4c-956a-99e11d8dbb76".to_string(),
        opened_at: Some("example".to_string()),
        provider: Some(Default::default()),
        provider_message_id: Some("example".to_string()),
        queued: Some(1),
        status: serde_json::from_value::<SendStatus>(serde_json::Value::String(
            "QUEUED".to_string(),
        ))
        .unwrap_or_default(),
        subject: Some("example".to_string()),
        template_id: Some("example".to_string()),
        template_version: Some(42),
        to: Some("example".to_string()),
        variant_key: Some("example".to_string()),
        workflow_id: Some("example".to_string()),
    })
}
