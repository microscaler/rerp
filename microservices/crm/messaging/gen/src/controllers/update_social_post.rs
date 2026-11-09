// User-owned controller for handler 'update_social_post'.

use crate::handlers::update_social_post::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::Media;
#[allow(unused_imports)]
use crate::handlers::types::PostStatus;
#[allow(unused_imports)]
use crate::handlers::types::PostTarget;

#[handler(UpdateSocialPostController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "body": "October release notes are out. Highlights in the thread.",
    //   "created_at": "2026-10-28T16:00:00Z",
    //   "created_by": "alex",
    //   "id": "de4620a8-882f-5f9c-88b0-d57c19355d18",
    //   "media": [],
    //   "requires_approval": false,
    //   "scheduled_at": "2026-11-02T09:30:00+02:00",
    //   "source": "alex",
    //   "status": "PUBLISHED",
    //   "targets": [
    //     {
    //       "account_id": "a41279e1-eaf5-5755-8860-1b6c8aebc4e3",
    //       "error": "Error",
    //       "permalink": "https://x.example/acme/0",
    //       "platform": "X",
    //       "status": "PUBLISHED"
    //     }
    //   ],
    //   "updated_at": "2026-10-28T16:00:00Z",
    //   "utm_campaign": "oct-product-update"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "body": "October release notes are out. Highlights in the thread.",
  "created_at": "2026-10-28T16:00:00Z",
  "created_by": "alex",
  "id": "de4620a8-882f-5f9c-88b0-d57c19355d18",
  "media": [],
  "requires_approval": false,
  "scheduled_at": "2026-11-02T09:30:00+02:00",
  "source": "alex",
  "status": "PUBLISHED",
  "targets": [
    {
      "account_id": "a41279e1-eaf5-5755-8860-1b6c8aebc4e3",
      "error": "Error",
      "permalink": "https://x.example/acme/0",
      "platform": "X",
      "status": "PUBLISHED"
    }
  ],
  "updated_at": "2026-10-28T16:00:00Z",
  "utm_campaign": "oct-product-update"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        approved_at: Some("example".to_string()),approved_by: Some("example".to_string()),body: "October release notes are out. Highlights in the thread.".to_string(),created_at: "2026-10-28T16:00:00Z".to_string(),created_by: Some("alex".to_string()),id: "de4620a8-882f-5f9c-88b0-d57c19355d18".to_string(),link_url: Some("example".to_string()),media: Some(vec![]),requires_approval: Some(false),scheduled_at: Some("2026-11-02T09:30:00+02:00".to_string()),skip_reason: Some("example".to_string()),source: Some("alex".to_string()),status: serde_json::from_value::<PostStatus>(serde_json::Value::String("PUBLISHED".to_string())).unwrap_or_default(),targets: vec![serde_json::from_value::<PostTarget>(serde_json::json!({"account_id":"a41279e1-eaf5-5755-8860-1b6c8aebc4e3","error":"Error","permalink":"https://x.example/acme/0","platform":"X","status":"PUBLISHED"})).unwrap_or_default()],updated_at: Some("2026-10-28T16:00:00Z".to_string()),utm_campaign: Some("oct-product-update".to_string()),utm_campaign_id: Some("example".to_string()),
    })
}
