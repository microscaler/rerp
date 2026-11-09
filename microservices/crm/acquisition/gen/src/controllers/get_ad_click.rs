// User-owned controller for handler 'get_ad_click'.

use crate::handlers::get_ad_click::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetAdClickController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "account_id": "bafc0e6d-6308-524f-9487-8d8424615bb7",
    //   "ad": "Ad",
    //   "ad_set": "Ad set",
    //   "at": "2026-11-05T16:00:00Z",
    //   "campaign": "Campaign",
    //   "click_id_param": "Click id param",
    //   "click_id_value": "Click id value",
    //   "consent_applied": "Consent applied",
    //   "contact_id": "057e8ff5-e1f2-53b3-bd11-7b45165fabe7",
    //   "contact_name": "Contact name",
    //   "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
    //   "landing_url": "Landing url",
    //   "platform": "Platform",
    //   "referrer": "Referrer",
    //   "touchpoints": 3,
    //   "utm": {
    //     "key": "Value"
    //   },
    //   "visitor_id": "ext_66efa929ba"
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "account_id": "bafc0e6d-6308-524f-9487-8d8424615bb7",
  "ad": "Ad",
  "ad_set": "Ad set",
  "at": "2026-11-05T16:00:00Z",
  "campaign": "Campaign",
  "click_id_param": "Click id param",
  "click_id_value": "Click id value",
  "consent_applied": "Consent applied",
  "contact_id": "057e8ff5-e1f2-53b3-bd11-7b45165fabe7",
  "contact_name": "Contact name",
  "id": "e67555dd-33f4-5fee-9da3-a019895219b1",
  "landing_url": "Landing url",
  "platform": "Platform",
  "referrer": "Referrer",
  "touchpoints": 3,
  "utm": {
    "key": "Value"
  },
  "visitor_id": "ext_66efa929ba"
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        account_id: Some("bafc0e6d-6308-524f-9487-8d8424615bb7".to_string()),
        ad: Some("Ad".to_string()),
        ad_set: Some("Ad set".to_string()),
        at: "2026-11-05T16:00:00Z".to_string(),
        campaign: Some("Campaign".to_string()),
        click_id_param: Some("Click id param".to_string()),
        click_id_value: Some("Click id value".to_string()),
        consent_applied: Some("Consent applied".to_string()),
        contact_id: Some("057e8ff5-e1f2-53b3-bd11-7b45165fabe7".to_string()),
        contact_name: Some("Contact name".to_string()),
        id: "e67555dd-33f4-5fee-9da3-a019895219b1".to_string(),
        landing_url: Some("Landing url".to_string()),
        platform: Some("Platform".to_string()),
        referrer: Some("Referrer".to_string()),
        touchpoints: Some(3),
        utm: Some(
            serde_json::from_str::<serde_json::Value>(r#"{"key":"Value"}"#)
                .unwrap_or(serde_json::Value::Null),
        ),
        visitor_id: Some("ext_66efa929ba".to_string()),
    })
}
