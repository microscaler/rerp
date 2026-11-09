// User-owned controller for handler 'list_ad_clicks'.

use crate::handlers::list_ad_clicks::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdClick;

#[handler(ListAdClicksController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "at": "2026-11-05T15:59:48Z",
    //       "campaign": "BF early access",
    //       "click_id_param": "gclid",
    //       "click_id_value": "CjwKCAiA…",
    //       "consent_applied": "FULL",
    //       "contact_id": "057e8ff5-e1f2-53b3-bd11-7b45165fabe7",
    //       "contact_name": "Contact name",
    //       "id": "2f26cc6b-ff21-5518-89b4-1bce65298ee7",
    //       "landing_url": "https://acme.example/early-access",
    //       "platform": "GOOGLE_ADS",
    //       "utm": {
    //         "utm_campaign": "bf-early-access",
    //         "utm_source": "google_ads"
    //       },
    //       "visitor_id": "v_9c12"
    //     },
    //     {
    //       "at": "2026-11-05T15:59:20Z",
    //       "campaign": "Warm-up BF",
    //       "click_id_param": "fbclid",
    //       "click_id_value": "IwAR2…",
    //       "consent_applied": "FULL",
    //       "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
    //       "contact_name": "Tom Okafor",
    //       "id": "eb6316fa-3fe5-5446-b4a3-d8ec2cf6cfef",
    //       "landing_url": "https://acme.example/early-access",
    //       "platform": "META",
    //       "utm": {
    //         "utm_campaign": "warm-up-bf",
    //         "utm_source": "meta"
    //       },
    //       "visitor_id": "v_2a77"
    //     },
    //     {
    //       "at": "2026-11-05T15:58:00Z",
    //       "campaign": "Webinar invite",
    //       "click_id_param": "li_fat_id",
    //       "click_id_value": "7a0b…",
    //       "consent_applied": "ANALYTICS_ONLY",
    //       "contact_id": "057e8ff5-e1f2-53b3-bd11-7b45165fabe7",
    //       "contact_name": "Contact name",
    //       "id": "d887cbdb-9b91-585e-83a8-d1801fb62a1a",
    //       "landing_url": "https://acme.example/early-access",
    //       "platform": "LINKEDIN",
    //       "utm": {
    //         "utm_campaign": "webinar-invite",
    //         "utm_source": "linkedin"
    //       },
    //       "visitor_id": "v_1e88"
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 4812
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "at": "2026-11-05T15:59:48Z",
      "campaign": "BF early access",
      "click_id_param": "gclid",
      "click_id_value": "CjwKCAiA…",
      "consent_applied": "FULL",
      "contact_id": "057e8ff5-e1f2-53b3-bd11-7b45165fabe7",
      "contact_name": "Contact name",
      "id": "2f26cc6b-ff21-5518-89b4-1bce65298ee7",
      "landing_url": "https://acme.example/early-access",
      "platform": "GOOGLE_ADS",
      "utm": {
        "utm_campaign": "bf-early-access",
        "utm_source": "google_ads"
      },
      "visitor_id": "v_9c12"
    },
    {
      "at": "2026-11-05T15:59:20Z",
      "campaign": "Warm-up BF",
      "click_id_param": "fbclid",
      "click_id_value": "IwAR2…",
      "consent_applied": "FULL",
      "contact_id": "7ed65e2d-0bd7-5e4e-9071-f4dac3456426",
      "contact_name": "Tom Okafor",
      "id": "eb6316fa-3fe5-5446-b4a3-d8ec2cf6cfef",
      "landing_url": "https://acme.example/early-access",
      "platform": "META",
      "utm": {
        "utm_campaign": "warm-up-bf",
        "utm_source": "meta"
      },
      "visitor_id": "v_2a77"
    },
    {
      "at": "2026-11-05T15:58:00Z",
      "campaign": "Webinar invite",
      "click_id_param": "li_fat_id",
      "click_id_value": "7a0b…",
      "consent_applied": "ANALYTICS_ONLY",
      "contact_id": "057e8ff5-e1f2-53b3-bd11-7b45165fabe7",
      "contact_name": "Contact name",
      "id": "d887cbdb-9b91-585e-83a8-d1801fb62a1a",
      "landing_url": "https://acme.example/early-access",
      "platform": "LINKEDIN",
      "utm": {
        "utm_campaign": "webinar-invite",
        "utm_source": "linkedin"
      },
      "visitor_id": "v_1e88"
    }
  ],
  "limit": 20,
  "page": 1,
  "total": 4812
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<AdClick>(serde_json::json!({"at":"2026-11-05T15:59:48Z","campaign":"BF early access","click_id_param":"gclid","click_id_value":"CjwKCAiA…","consent_applied":"FULL","contact_id":"057e8ff5-e1f2-53b3-bd11-7b45165fabe7","contact_name":"Contact name","id":"2f26cc6b-ff21-5518-89b4-1bce65298ee7","landing_url":"https://acme.example/early-access","platform":"GOOGLE_ADS","utm":{"utm_campaign":"bf-early-access","utm_source":"google_ads"},"visitor_id":"v_9c12"})).unwrap_or_default(), serde_json::from_value::<AdClick>(serde_json::json!({"at":"2026-11-05T15:59:20Z","campaign":"Warm-up BF","click_id_param":"fbclid","click_id_value":"IwAR2…","consent_applied":"FULL","contact_id":"7ed65e2d-0bd7-5e4e-9071-f4dac3456426","contact_name":"Tom Okafor","id":"eb6316fa-3fe5-5446-b4a3-d8ec2cf6cfef","landing_url":"https://acme.example/early-access","platform":"META","utm":{"utm_campaign":"warm-up-bf","utm_source":"meta"},"visitor_id":"v_2a77"})).unwrap_or_default(), serde_json::from_value::<AdClick>(serde_json::json!({"at":"2026-11-05T15:58:00Z","campaign":"Webinar invite","click_id_param":"li_fat_id","click_id_value":"7a0b…","consent_applied":"ANALYTICS_ONLY","contact_id":"057e8ff5-e1f2-53b3-bd11-7b45165fabe7","contact_name":"Contact name","id":"d887cbdb-9b91-585e-83a8-d1801fb62a1a","landing_url":"https://acme.example/early-access","platform":"LINKEDIN","utm":{"utm_campaign":"webinar-invite","utm_source":"linkedin"},"visitor_id":"v_1e88"})).unwrap_or_default()],limit: 20,page: 1,total: 4812,
    })
}
