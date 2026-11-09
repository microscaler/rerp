// User-owned controller for handler 'resend_confirmation'.

use crate::handlers::resend_confirmation::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(ResendConfirmationController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "confirmation_count": 3,
    //   "confirmation_sent_at": "2026-11-05T16:00:00Z",
    //   "confirmed_at": "2026-11-05T16:00:00Z",
    //   "decided_at": "2026-11-05T16:00:00Z",
    //   "email_masked": "Email masked",
    //   "expires_at": "2026-11-05T16:00:00Z",
    //   "form_id": "925a878b-6c34-54c3-aae1-7b3a051667f8",
    //   "form_name": "Form name",
    //   "id": "19659456-e0be-5b35-8762-ea6182cdf0cb",
    //   "kind": "SUBSCRIPTION",
    //   "reject_reason": "HONEYPOT",
    //   "status": "PENDING",
    //   "submitted_at": "2026-11-05T16:00:00Z"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "confirmation_count": 3,
  "confirmation_sent_at": "2026-11-05T16:00:00Z",
  "confirmed_at": "2026-11-05T16:00:00Z",
  "decided_at": "2026-11-05T16:00:00Z",
  "email_masked": "Email masked",
  "expires_at": "2026-11-05T16:00:00Z",
  "form_id": "925a878b-6c34-54c3-aae1-7b3a051667f8",
  "form_name": "Form name",
  "id": "19659456-e0be-5b35-8762-ea6182cdf0cb",
  "kind": "SUBSCRIPTION",
  "reject_reason": "HONEYPOT",
  "status": "PENDING",
  "submitted_at": "2026-11-05T16:00:00Z"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        confirmation_count: Some(3),
        confirmation_sent_at: Some("2026-11-05T16:00:00Z".to_string()),
        confirmed_at: Some("2026-11-05T16:00:00Z".to_string()),
        decided_at: Some("2026-11-05T16:00:00Z".to_string()),
        email_masked: Some("Email masked".to_string()),
        expires_at: Some("2026-11-05T16:00:00Z".to_string()),
        form_id: Some("925a878b-6c34-54c3-aae1-7b3a051667f8".to_string()),
        form_name: Some("Form name".to_string()),
        id: "19659456-e0be-5b35-8762-ea6182cdf0cb".to_string(),
        kind: "SUBSCRIPTION".to_string(),
        reject_reason: Some("HONEYPOT".to_string()),
        status: "PENDING".to_string(),
        submitted_at: "2026-11-05T16:00:00Z".to_string(),
    })
}
