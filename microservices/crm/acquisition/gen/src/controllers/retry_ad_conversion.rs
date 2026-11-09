// User-owned controller for handler 'retry_ad_conversion'.

use crate::handlers::retry_ad_conversion::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RetryAdConversionController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "account_id": "bafc0e6d-6308-524f-9487-8d8424615bb7",
    //   "click_id_param": "Click id param",
    //   "contact_id": "057e8ff5-e1f2-53b3-bd11-7b45165fabe7",
    //   "contact_name": "Contact name",
    //   "currency": "EUR",
    //   "deal_id": "15ecb045-46df-5097-88d9-d5715fe6c9ab",
    //   "error": "Error",
    //   "event": "Event",
    //   "external_event_name": "External event name",
    //   "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
    //   "identifiers_sent": [
    //     "Identifiers sent"
    //   ],
    //   "occurred_at": "2026-11-05T16:00:00Z",
    //   "platform": "Platform",
    //   "platform_response": "Platform response",
    //   "rule_id": "c51a9a99-1aeb-5639-a0a5-da7052737b15",
    //   "sent_at": "2026-11-05T16:00:00Z",
    //   "status": "QUEUED",
    //   "value": "1250.00"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "account_id": "bafc0e6d-6308-524f-9487-8d8424615bb7",
  "click_id_param": "Click id param",
  "contact_id": "057e8ff5-e1f2-53b3-bd11-7b45165fabe7",
  "contact_name": "Contact name",
  "currency": "EUR",
  "deal_id": "15ecb045-46df-5097-88d9-d5715fe6c9ab",
  "error": "Error",
  "event": "Event",
  "external_event_name": "External event name",
  "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
  "identifiers_sent": [
    "Identifiers sent"
  ],
  "occurred_at": "2026-11-05T16:00:00Z",
  "platform": "Platform",
  "platform_response": "Platform response",
  "rule_id": "c51a9a99-1aeb-5639-a0a5-da7052737b15",
  "sent_at": "2026-11-05T16:00:00Z",
  "status": "QUEUED",
  "value": "1250.00"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        account_id: Some("bafc0e6d-6308-524f-9487-8d8424615bb7".to_string()),
        click_id_param: Some("Click id param".to_string()),
        contact_id: Some("057e8ff5-e1f2-53b3-bd11-7b45165fabe7".to_string()),
        contact_name: Some("Contact name".to_string()),
        currency: Some("EUR".to_string()),
        deal_id: Some("15ecb045-46df-5097-88d9-d5715fe6c9ab".to_string()),
        error: Some("Error".to_string()),
        event: Some("Event".to_string()),
        external_event_name: Some("External event name".to_string()),
        id: "e67555dd-33f4-5fee-9da3-a019895219b1".to_string(),
        identifiers_sent: Some(vec!["Identifiers sent".to_string()]),
        occurred_at: Some("2026-11-05T16:00:00Z".to_string()),
        platform: Some("Platform".to_string()),
        platform_response: Some("Platform response".to_string()),
        rule_id: Some("c51a9a99-1aeb-5639-a0a5-da7052737b15".to_string()),
        sent_at: Some("2026-11-05T16:00:00Z".to_string()),
        status: "QUEUED".to_string(),
        value: Some("1250.00".to_string()),
    })
}
