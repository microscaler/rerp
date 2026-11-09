// User-owned controller for handler 'get_sending_providers'.

use crate::handlers::get_sending_providers::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetSendingProvidersController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "channels": [
    //     {
    //       "key": "sms",
    //       "label": "SMS (Twilio)",
    //       "status": "NOT_CONNECTED"
    //     },
    //     {
    //       "key": "whatsapp",
    //       "label": "WhatsApp (Meta)",
    //       "status": "NOT_CONNECTED"
    //     },
    //     {
    //       "key": "social",
    //       "label": "X · LinkedIn · Threads",
    //       "status": "CONNECTED"
    //     }
    //   ],
    //   "email": {
    //     "adapter": "Amazon SES",
    //     "alternatives": [
    //       "Postmark",
    //       "Resend"
    //     ],
    //     "credentials": "Workload identity · no stored keys",
    //     "daily_quota": 50000,
    //     "region": "eu-central-1",
    //     "used_today": 12410,
    //     "webhook_last_event_at": "2026-11-05T15:59:56Z",
    //     "webhook_status": "Connected"
    //   }
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "channels": [
    {
      "key": "sms",
      "label": "SMS (Twilio)",
      "status": "NOT_CONNECTED"
    },
    {
      "key": "whatsapp",
      "label": "WhatsApp (Meta)",
      "status": "NOT_CONNECTED"
    },
    {
      "key": "social",
      "label": "X · LinkedIn · Threads",
      "status": "CONNECTED"
    }
  ],
  "email": {
    "adapter": "Amazon SES",
    "alternatives": [
      "Postmark",
      "Resend"
    ],
    "credentials": "Workload identity · no stored keys",
    "daily_quota": 50000,
    "region": "eu-central-1",
    "used_today": 12410,
    "webhook_last_event_at": "2026-11-05T15:59:56Z",
    "webhook_status": "Connected"
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
        channels: Some(vec![serde_json::from_str::<serde_json::Value>(r#"{"key":"sms","label":"SMS (Twilio)","status":"NOT_CONNECTED"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"key":"whatsapp","label":"WhatsApp (Meta)","status":"NOT_CONNECTED"}"#).unwrap_or(serde_json::Value::Null), serde_json::from_str::<serde_json::Value>(r#"{"key":"social","label":"X · LinkedIn · Threads","status":"CONNECTED"}"#).unwrap_or(serde_json::Value::Null)]),email: Some(serde_json::from_str::<serde_json::Value>(r#"{"adapter":"Amazon SES","alternatives":["Postmark","Resend"],"credentials":"Workload identity · no stored keys","daily_quota":50000,"region":"eu-central-1","used_today":12410,"webhook_last_event_at":"2026-11-05T15:59:56Z","webhook_status":"Connected"}"#).unwrap_or(serde_json::Value::Null)),
    })
}
