// User-owned controller for handler 'preview_ad_audience'.

use crate::handlers::preview_ad_audience::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(PreviewAdAudienceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "below_platform_minimum": false,
    //   "eligible": 2983,
    //   "estimated_match_rate": 0.61,
    //   "estimated_matched": 1820,
    //   "excluded": {
    //     "exclude_source": 1180,
    //     "no_consent": 104,
    //     "suppressed": 96,
    //     "unsubscribed": 212
    //   },
    //   "identifier_coverage": {
    //     "EMAIL": 1.0,
    //     "FIRST_NAME": 0.94,
    //     "LAST_NAME": 0.94,
    //     "PHONE": 0.38,
    //     "POSTCODE": 0.22
    //   },
    //   "source_count": 5102
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "below_platform_minimum": false,
  "eligible": 2983,
  "estimated_match_rate": 0.61,
  "estimated_matched": 1820,
  "excluded": {
    "exclude_source": 1180,
    "no_consent": 104,
    "suppressed": 96,
    "unsubscribed": 212
  },
  "identifier_coverage": {
    "EMAIL": 1.0,
    "FIRST_NAME": 0.94,
    "LAST_NAME": 0.94,
    "PHONE": 0.38,
    "POSTCODE": 0.22
  },
  "source_count": 5102
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        below_platform_minimum: Some(false),
        eligible: Some(2983),
        estimated_match_rate: Some(0.61),
        estimated_matched: Some(1820),
        excluded: Some(
            serde_json::from_str::<serde_json::Value>(
                r#"{"exclude_source":1180,"no_consent":104,"suppressed":96,"unsubscribed":212}"#,
            )
            .unwrap_or(serde_json::Value::Null),
        ),
        identifier_coverage: Some(
            serde_json::from_str::<serde_json::Value>(
                r#"{"EMAIL":1.0,"FIRST_NAME":0.94,"LAST_NAME":0.94,"PHONE":0.38,"POSTCODE":0.22}"#,
            )
            .unwrap_or(serde_json::Value::Null),
        ),
        source_count: Some(5102),
    })
}
