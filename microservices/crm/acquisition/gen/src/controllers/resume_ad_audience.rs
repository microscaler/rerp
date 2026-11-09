// User-owned controller for handler 'resume_ad_audience'.

use crate::handlers::resume_ad_audience::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdAudienceSource;

#[handler(ResumeAdAudienceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
    //   "account_name": "Acme · Meta EU",
    //   "consent_basis": "CONSENT",
    //   "created_at": "2026-10-24T16:00:00Z",
    //   "created_by": "alex",
    //   "exclude_source": {
    //     "kind": "SEGMENT",
    //     "ref_id": "3fa577a2-af26-5107-ad12-4ae022268d0f"
    //   },
    //   "external_audience_id": "2389…",
    //   "id": "b6d841b8-e819-50bb-ba55-f9f0c0e679bf",
    //   "identifiers": [
    //     "EMAIL",
    //     "PHONE",
    //     "FIRST_NAME"
    //   ],
    //   "last_error": "Last error",
    //   "lookalike": {
    //     "country": "DE",
    //     "enabled": false,
    //     "ratio_percent": 2
    //   },
    //   "mode": "CONTINUOUS",
    //   "name": "Warm-up · Black Friday early access",
    //   "platform": "META",
    //   "purpose": "WARM_UP",
    //   "source": {
    //     "kind": "SEGMENT",
    //     "ref_id": "20df9eb1-f769-5766-b664-2f6b6529022c"
    //   },
    //   "stats": {
    //     "eligible": 3112,
    //     "last_sync_at": "2026-11-05T15:48:00Z",
    //     "match_rate": 0.61,
    //     "matched": 1900,
    //     "next_sync_at": "2026-11-05T16:48:00Z",
    //     "removed_total": 288,
    //     "source_count": 5102,
    //     "uploaded": 3112
    //   },
    //   "status": "ACTIVE",
    //   "suppress_on": [
    //     "UNSUBSCRIBED",
    //     "SUPPRESSED",
    //     "ERASED"
    //   ],
    //   "sync_interval_minutes": 60,
    //   "updated_at": "2026-11-05T15:48:00Z",
    //   "warm_up": {
    //     "days_before_send": 10,
    //     "frequency_cap_hint": 4,
    //     "remove_on": "EMAIL_CLICKED",
    //     "window_days": 14
    //   }
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
  "account_name": "Acme · Meta EU",
  "consent_basis": "CONSENT",
  "created_at": "2026-10-24T16:00:00Z",
  "created_by": "alex",
  "exclude_source": {
    "kind": "SEGMENT",
    "ref_id": "3fa577a2-af26-5107-ad12-4ae022268d0f"
  },
  "external_audience_id": "2389…",
  "id": "b6d841b8-e819-50bb-ba55-f9f0c0e679bf",
  "identifiers": [
    "EMAIL",
    "PHONE",
    "FIRST_NAME"
  ],
  "last_error": "Last error",
  "lookalike": {
    "country": "DE",
    "enabled": false,
    "ratio_percent": 2
  },
  "mode": "CONTINUOUS",
  "name": "Warm-up · Black Friday early access",
  "platform": "META",
  "purpose": "WARM_UP",
  "source": {
    "kind": "SEGMENT",
    "ref_id": "20df9eb1-f769-5766-b664-2f6b6529022c"
  },
  "stats": {
    "eligible": 3112,
    "last_sync_at": "2026-11-05T15:48:00Z",
    "match_rate": 0.61,
    "matched": 1900,
    "next_sync_at": "2026-11-05T16:48:00Z",
    "removed_total": 288,
    "source_count": 5102,
    "uploaded": 3112
  },
  "status": "ACTIVE",
  "suppress_on": [
    "UNSUBSCRIBED",
    "SUPPRESSED",
    "ERASED"
  ],
  "sync_interval_minutes": 60,
  "updated_at": "2026-11-05T15:48:00Z",
  "warm_up": {
    "days_before_send": 10,
    "frequency_cap_hint": 4,
    "remove_on": "EMAIL_CLICKED",
    "window_days": 14
  }
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        account_id: "ded95315-6c77-51a0-b8ea-66b76fada21a".to_string(),account_name: Some("Acme · Meta EU".to_string()),consent_basis: Some("CONSENT".to_string()),created_at: Some("2026-10-24T16:00:00Z".to_string()),created_by: Some("alex".to_string()),exclude_source: Some(serde_json::from_value::<AdAudienceSource>(serde_json::json!({"kind":"SEGMENT","ref_id":"3fa577a2-af26-5107-ad12-4ae022268d0f"})).unwrap_or_default()),external_audience_id: Some("2389…".to_string()),id: "b6d841b8-e819-50bb-ba55-f9f0c0e679bf".to_string(),identifiers: Some(vec!["EMAIL".to_string(), "PHONE".to_string(), "FIRST_NAME".to_string()]),last_error: Some("Last error".to_string()),lookalike: Some(serde_json::from_str::<serde_json::Value>(r#"{"country":"DE","enabled":false,"ratio_percent":2}"#).unwrap_or(serde_json::Value::Null)),mode: Some("CONTINUOUS".to_string()),name: "Warm-up · Black Friday early access".to_string(),platform: "META".to_string(),purpose: Some("WARM_UP".to_string()),source: serde_json::from_value::<AdAudienceSource>(serde_json::json!({"kind":"SEGMENT","ref_id":"20df9eb1-f769-5766-b664-2f6b6529022c"})).unwrap_or_default(),stats: Some(serde_json::from_str::<serde_json::Value>(r#"{"eligible":3112,"last_sync_at":"2026-11-05T15:48:00Z","match_rate":0.61,"matched":1900,"next_sync_at":"2026-11-05T16:48:00Z","removed_total":288,"source_count":5102,"uploaded":3112}"#).unwrap_or(serde_json::Value::Null)),status: "ACTIVE".to_string(),suppress_on: Some(vec!["UNSUBSCRIBED".to_string(), "SUPPRESSED".to_string(), "ERASED".to_string()]),sync_interval_minutes: Some(60),updated_at: Some("2026-11-05T15:48:00Z".to_string()),warm_up: Some(serde_json::from_str::<serde_json::Value>(r#"{"days_before_send":10,"frequency_cap_hint":4,"remove_on":"EMAIL_CLICKED","window_days":14}"#).unwrap_or(serde_json::Value::Null)),
    })
}
