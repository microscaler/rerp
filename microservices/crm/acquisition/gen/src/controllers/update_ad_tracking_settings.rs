// User-owned controller for handler 'update_ad_tracking_settings'.

use crate::handlers::update_ad_tracking_settings::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateAdTrackingSettingsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "capture_referrer": true,
    //   "capture_utm": true,
    //   "click_id_params": [
    //     "gclid",
    //     "wbraid",
    //     "gbraid"
    //   ],
    //   "consent": {
    //     "cmp_signal": "TCF",
    //     "cookieless_pings": true,
    //     "mode": "REGION",
    //     "strict_regions": [
    //       "EU",
    //       "EEA",
    //       "UK"
    //     ]
    //   },
    //   "cookie": {
    //     "localstorage_mirror": true,
    //     "name": "_pw_vid",
    //     "same_site": "Lax",
    //     "server_set": true,
    //     "ttl_days": 400
    //   },
    //   "default_attribution_model": "last_touch",
    //   "default_window_days": 30,
    //   "fingerprint": {
    //     "enabled": true,
    //     "ip_truncation": "LAST_OCTET",
    //     "signals": [
    //       "IP_PREFIX",
    //       "USER_AGENT",
    //       "SCREEN"
    //     ]
    //   },
    //   "identify_on": [
    //     "FORM_SUBMIT",
    //     "EMAIL_LINK",
    //     "SMS_LINK"
    //   ],
    //   "link_decoration": {
    //     "contact_token": true,
    //     "enabled": true,
    //     "params": [
    //       "pw_cid",
    //       "utm_source",
    //       "utm_medium"
    //     ]
    //   },
    //   "own_click_id_param": "pw_cid",
    //   "updated_at": "2026-11-02T16:00:00Z"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "capture_referrer": true,
  "capture_utm": true,
  "click_id_params": [
    "gclid",
    "wbraid",
    "gbraid"
  ],
  "consent": {
    "cmp_signal": "TCF",
    "cookieless_pings": true,
    "mode": "REGION",
    "strict_regions": [
      "EU",
      "EEA",
      "UK"
    ]
  },
  "cookie": {
    "localstorage_mirror": true,
    "name": "_pw_vid",
    "same_site": "Lax",
    "server_set": true,
    "ttl_days": 400
  },
  "default_attribution_model": "last_touch",
  "default_window_days": 30,
  "fingerprint": {
    "enabled": true,
    "ip_truncation": "LAST_OCTET",
    "signals": [
      "IP_PREFIX",
      "USER_AGENT",
      "SCREEN"
    ]
  },
  "identify_on": [
    "FORM_SUBMIT",
    "EMAIL_LINK",
    "SMS_LINK"
  ],
  "link_decoration": {
    "contact_token": true,
    "enabled": true,
    "params": [
      "pw_cid",
      "utm_source",
      "utm_medium"
    ]
  },
  "own_click_id_param": "pw_cid",
  "updated_at": "2026-11-02T16:00:00Z"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        capture_referrer: Some(true),capture_utm: Some(true),click_id_params: Some(vec!["gclid".to_string(), "wbraid".to_string(), "gbraid".to_string()]),consent: Some(serde_json::from_str::<serde_json::Value>(r#"{"cmp_signal":"TCF","cookieless_pings":true,"mode":"REGION","strict_regions":["EU","EEA","UK"]}"#).unwrap_or(serde_json::Value::Null)),cookie: Some(serde_json::from_str::<serde_json::Value>(r#"{"localstorage_mirror":true,"name":"_pw_vid","same_site":"Lax","server_set":true,"ttl_days":400}"#).unwrap_or(serde_json::Value::Null)),default_attribution_model: Some("last_touch".to_string()),default_window_days: Some(30),fingerprint: Some(serde_json::from_str::<serde_json::Value>(r#"{"enabled":true,"ip_truncation":"LAST_OCTET","signals":["IP_PREFIX","USER_AGENT","SCREEN"]}"#).unwrap_or(serde_json::Value::Null)),identify_on: Some(vec!["FORM_SUBMIT".to_string(), "EMAIL_LINK".to_string(), "SMS_LINK".to_string()]),link_decoration: Some(serde_json::from_str::<serde_json::Value>(r#"{"contact_token":true,"enabled":true,"params":["pw_cid","utm_source","utm_medium"]}"#).unwrap_or(serde_json::Value::Null)),own_click_id_param: Some("pw_cid".to_string()),updated_at: Some("2026-11-02T16:00:00Z".to_string()),
    })
}
