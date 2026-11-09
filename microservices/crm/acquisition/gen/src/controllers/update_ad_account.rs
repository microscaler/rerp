// User-owned controller for handler 'update_ad_account'.

use crate::handlers::update_ad_account::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateAdAccountController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "audience_count": 4,
    //   "capabilities": [
    //     "AUDIENCES",
    //     "CONVERSIONS",
    //     "SPEND"
    //   ],
    //   "connected_by": "alex",
    //   "conversion_endpoint": "META_CAPI",
    //   "created_at": "2026-07-08T16:00:00Z",
    //   "default_currency": "EUR",
    //   "external_account_id": "act_1092884411",
    //   "id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
    //   "last_error": "Last error",
    //   "last_spend_import_at": "2026-11-05T10:00:00Z",
    //   "last_sync_at": "2026-11-05T15:48:00Z",
    //   "match_rate_last": 0.61,
    //   "name": "Acme · Meta EU",
    //   "platform": "META",
    //   "spend_30d": 2140,
    //   "status": "CONNECTED"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "audience_count": 4,
  "capabilities": [
    "AUDIENCES",
    "CONVERSIONS",
    "SPEND"
  ],
  "connected_by": "alex",
  "conversion_endpoint": "META_CAPI",
  "created_at": "2026-07-08T16:00:00Z",
  "default_currency": "EUR",
  "external_account_id": "act_1092884411",
  "id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
  "last_error": "Last error",
  "last_spend_import_at": "2026-11-05T10:00:00Z",
  "last_sync_at": "2026-11-05T15:48:00Z",
  "match_rate_last": 0.61,
  "name": "Acme · Meta EU",
  "platform": "META",
  "spend_30d": 2140,
  "status": "CONNECTED"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        audience_count: Some(4),
        authorize_url: Some("example".to_string()),
        capabilities: Some(vec![
            "AUDIENCES".to_string(),
            "CONVERSIONS".to_string(),
            "SPEND".to_string(),
        ]),
        connected_by: Some("alex".to_string()),
        conversion_endpoint: Some("META_CAPI".to_string()),
        created_at: Some("2026-07-08T16:00:00Z".to_string()),
        default_currency: Some("EUR".to_string()),
        external_account_id: Some("act_1092884411".to_string()),
        id: "ded95315-6c77-51a0-b8ea-66b76fada21a".to_string(),
        last_error: Some("Last error".to_string()),
        last_spend_import_at: Some("2026-11-05T10:00:00Z".to_string()),
        last_sync_at: Some("2026-11-05T15:48:00Z".to_string()),
        match_rate_last: Some(0.61),
        name: "Acme · Meta EU".to_string(),
        pixel_id: Some("example".to_string()),
        platform: "META".to_string(),
        spend_30d: Some(2140),
        status: "CONNECTED".to_string(),
    })
}
