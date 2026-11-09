// User-owned controller for handler 'update_list'.

use crate::handlers::update_list::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateListController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "channel": "EMAIL",
    //   "confirmation_template_id": "be01905c-70da-59d9-b7ad-e59f4d2f4109",
    //   "created_at": "2026-03-10T16:00:00Z",
    //   "description": "Monthly product news",
    //   "double_opt_in": true,
    //   "growth_30d": [
    //     8,
    //     10,
    //     12
    //   ],
    //   "id": "09f5b2f9-2adc-5edd-a6fd-c20675680671",
    //   "is_archived": false,
    //   "name": "Newsletter",
    //   "pending_count": 144,
    //   "postal_address": "Acme Co, 12 Example Street, 10115 Berlin, Germany",
    //   "reminder_text": "You are receiving this because you signed up on our website.",
    //   "sender_id": "37371e40-5aeb-5ad2-a9e6-389a2d6f0ca2",
    //   "subscriber_count": 9870,
    //   "unsubscribed_count": 812
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "channel": "EMAIL",
  "confirmation_template_id": "be01905c-70da-59d9-b7ad-e59f4d2f4109",
  "created_at": "2026-03-10T16:00:00Z",
  "description": "Monthly product news",
  "double_opt_in": true,
  "growth_30d": [
    8,
    10,
    12
  ],
  "id": "09f5b2f9-2adc-5edd-a6fd-c20675680671",
  "is_archived": false,
  "name": "Newsletter",
  "pending_count": 144,
  "postal_address": "Acme Co, 12 Example Street, 10115 Berlin, Germany",
  "reminder_text": "You are receiving this because you signed up on our website.",
  "sender_id": "37371e40-5aeb-5ad2-a9e6-389a2d6f0ca2",
  "subscriber_count": 9870,
  "unsubscribed_count": 812
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        channel: Some("EMAIL".to_string()),
        confirmation_template_id: Some("be01905c-70da-59d9-b7ad-e59f4d2f4109".to_string()),
        created_at: "2026-03-10T16:00:00Z".to_string(),
        description: Some("Monthly product news".to_string()),
        double_opt_in: true,
        growth_30d: Some(vec![8, 10, 12]),
        id: "09f5b2f9-2adc-5edd-a6fd-c20675680671".to_string(),
        is_archived: Some(false),
        name: "Newsletter".to_string(),
        pending_count: Some(144),
        postal_address: Some("Acme Co, 12 Example Street, 10115 Berlin, Germany".to_string()),
        reminder_text: Some(
            "You are receiving this because you signed up on our website.".to_string(),
        ),
        sender_id: "37371e40-5aeb-5ad2-a9e6-389a2d6f0ca2".to_string(),
        subscriber_count: Some(9870),
        unsubscribed_count: Some(812),
        updated_at: Some("example".to_string()),
    })
}
