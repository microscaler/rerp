// User-owned controller for handler 'list_ad_audiences'.

use crate::handlers::list_ad_audiences::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdAudience;

#[handler(ListAdAudiencesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
    //       "consent_basis": "CONSENT",
    //       "created_at": "2026-10-24T16:00:00Z",
    //       "created_by": "alex",
    //       "exclude_source": {
    //         "kind": "SEGMENT",
    //         "ref_id": "3fa577a2-af26-5107-ad12-4ae022268d0f"
    //       },
    //       "external_audience_id": "2389…",
    //       "id": "b6d841b8-e819-50bb-ba55-f9f0c0e679bf",
    //       "identifiers": [
    //         "EMAIL",
    //         "PHONE",
    //         "FIRST_NAME"
    //       ],
    //       "last_error": "Last error",
    //       "lookalike": {
    //         "country": "DE",
    //         "enabled": false,
    //         "ratio_percent": 2
    //       },
    //       "mode": "CONTINUOUS",
    //       "name": "Warm-up · Black Friday early access",
    //       "platform": "META",
    //       "purpose": "WARM_UP",
    //       "source": {
    //         "kind": "SEGMENT",
    //         "ref_id": "20df9eb1-f769-5766-b664-2f6b6529022c"
    //       },
    //       "stats": {
    //         "eligible": 3112,
    //         "last_sync_at": "2026-11-05T15:48:00Z",
    //         "match_rate": 0.61,
    //         "matched": 1900,
    //         "next_sync_at": "2026-11-05T16:48:00Z",
    //         "removed_total": 288,
    //         "source_count": 5102,
    //         "uploaded": 3112
    //       },
    //       "status": "ACTIVE",
    //       "suppress_on": [
    //         "UNSUBSCRIBED",
    //         "SUPPRESSED",
    //         "ERASED"
    //       ],
    //       "sync_interval_minutes": 60,
    //       "updated_at": "2026-11-05T15:48:00Z",
    //       "warm_up": {
    //         "days_before_send": 10,
    //         "frequency_cap_hint": 4,
    //         "remove_on": "EMAIL_CLICKED",
    //         "window_days": 14
    //       }
    //     },
    //     {
    //       "account_id": "6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa",
    //       "consent_basis": "CONSENT",
    //       "created_at": "2026-10-23T16:00:00Z",
    //       "created_by": "alex",
    //       "exclude_source": {
    //         "kind": "SEGMENT"
    //       },
    //       "external_audience_id": "2406…",
    //       "id": "bbab0796-23f0-59cb-841b-ce85c5b69aae",
    //       "identifiers": [
    //         "EMAIL"
    //       ],
    //       "last_error": "Last error",
    //       "lookalike": {
    //         "country": "DE",
    //         "enabled": false,
    //         "ratio_percent": 2
    //       },
    //       "mode": "CONTINUOUS",
    //       "name": "Warm-up · Webinar invite (LinkedIn)",
    //       "platform": "LINKEDIN",
    //       "purpose": "WARM_UP",
    //       "source": {
    //         "kind": "LIST",
    //         "ref_id": "09f5b2f9-2adc-5edd-a6fd-c20675680671"
    //       },
    //       "stats": {
    //         "eligible": 1804,
    //         "last_sync_at": "2026-11-05T15:00:00Z",
    //         "match_rate": 0.68,
    //         "matched": 1226,
    //         "next_sync_at": "2026-11-05T16:48:00Z",
    //         "removed_total": 40,
    //         "source_count": 2604,
    //         "uploaded": 1804
    //       },
    //       "status": "ACTIVE",
    //       "suppress_on": [
    //         "UNSUBSCRIBED",
    //         "SUPPRESSED",
    //         "ERASED"
    //       ],
    //       "sync_interval_minutes": 60,
    //       "updated_at": "2026-11-05T15:00:00Z",
    //       "warm_up": {
    //         "days_before_send": 10,
    //         "frequency_cap_hint": 4,
    //         "remove_on": "EMAIL_CLICKED",
    //         "window_days": 14
    //       }
    //     },
    //     {
    //       "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
    //       "consent_basis": "CONSENT",
    //       "created_at": "2026-10-22T16:00:00Z",
    //       "created_by": "alex",
    //       "exclude_source": {
    //         "kind": "SEGMENT"
    //       },
    //       "external_audience_id": "2423…",
    //       "id": "fa613142-3567-5941-ad4b-02a09f597962",
    //       "identifiers": [
    //         "EMAIL"
    //       ],
    //       "last_error": "Last error",
    //       "lookalike": {
    //         "country": "DE",
    //         "enabled": false,
    //         "ratio_percent": 2
    //       },
    //       "mode": "CONTINUOUS",
    //       "name": "Retarget · pricing visitors 30 d",
    //       "platform": "META",
    //       "purpose": "RETARGET",
    //       "source": {
    //         "kind": "SEGMENT",
    //         "ref_id": "20df9eb1-f769-5766-b664-2f6b6529022c"
    //       },
    //       "stats": {
    //         "eligible": 2206,
    //         "last_sync_at": "2026-11-05T15:48:00Z",
    //         "match_rate": 0.61,
    //         "matched": 1346,
    //         "next_sync_at": "2026-11-05T16:48:00Z",
    //         "removed_total": 40,
    //         "source_count": 2410,
    //         "uploaded": 2206
    //       },
    //       "status": "ACTIVE",
    //       "suppress_on": [
    //         "UNSUBSCRIBED",
    //         "SUPPRESSED",
    //         "ERASED"
    //       ],
    //       "sync_interval_minutes": 60,
    //       "updated_at": "2026-11-05T15:48:00Z",
    //       "warm_up": {}
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 6
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
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
    },
    {
      "account_id": "6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa",
      "consent_basis": "CONSENT",
      "created_at": "2026-10-23T16:00:00Z",
      "created_by": "alex",
      "exclude_source": {
        "kind": "SEGMENT"
      },
      "external_audience_id": "2406…",
      "id": "bbab0796-23f0-59cb-841b-ce85c5b69aae",
      "identifiers": [
        "EMAIL"
      ],
      "last_error": "Last error",
      "lookalike": {
        "country": "DE",
        "enabled": false,
        "ratio_percent": 2
      },
      "mode": "CONTINUOUS",
      "name": "Warm-up · Webinar invite (LinkedIn)",
      "platform": "LINKEDIN",
      "purpose": "WARM_UP",
      "source": {
        "kind": "LIST",
        "ref_id": "09f5b2f9-2adc-5edd-a6fd-c20675680671"
      },
      "stats": {
        "eligible": 1804,
        "last_sync_at": "2026-11-05T15:00:00Z",
        "match_rate": 0.68,
        "matched": 1226,
        "next_sync_at": "2026-11-05T16:48:00Z",
        "removed_total": 40,
        "source_count": 2604,
        "uploaded": 1804
      },
      "status": "ACTIVE",
      "suppress_on": [
        "UNSUBSCRIBED",
        "SUPPRESSED",
        "ERASED"
      ],
      "sync_interval_minutes": 60,
      "updated_at": "2026-11-05T15:00:00Z",
      "warm_up": {
        "days_before_send": 10,
        "frequency_cap_hint": 4,
        "remove_on": "EMAIL_CLICKED",
        "window_days": 14
      }
    },
    {
      "account_id": "ded95315-6c77-51a0-b8ea-66b76fada21a",
      "consent_basis": "CONSENT",
      "created_at": "2026-10-22T16:00:00Z",
      "created_by": "alex",
      "exclude_source": {
        "kind": "SEGMENT"
      },
      "external_audience_id": "2423…",
      "id": "fa613142-3567-5941-ad4b-02a09f597962",
      "identifiers": [
        "EMAIL"
      ],
      "last_error": "Last error",
      "lookalike": {
        "country": "DE",
        "enabled": false,
        "ratio_percent": 2
      },
      "mode": "CONTINUOUS",
      "name": "Retarget · pricing visitors 30 d",
      "platform": "META",
      "purpose": "RETARGET",
      "source": {
        "kind": "SEGMENT",
        "ref_id": "20df9eb1-f769-5766-b664-2f6b6529022c"
      },
      "stats": {
        "eligible": 2206,
        "last_sync_at": "2026-11-05T15:48:00Z",
        "match_rate": 0.61,
        "matched": 1346,
        "next_sync_at": "2026-11-05T16:48:00Z",
        "removed_total": 40,
        "source_count": 2410,
        "uploaded": 2206
      },
      "status": "ACTIVE",
      "suppress_on": [
        "UNSUBSCRIBED",
        "SUPPRESSED",
        "ERASED"
      ],
      "sync_interval_minutes": 60,
      "updated_at": "2026-11-05T15:48:00Z",
      "warm_up": {}
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 6
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<AdAudience>(serde_json::json!({"account_id":"ded95315-6c77-51a0-b8ea-66b76fada21a","consent_basis":"CONSENT","created_at":"2026-10-24T16:00:00Z","created_by":"alex","exclude_source":{"kind":"SEGMENT","ref_id":"3fa577a2-af26-5107-ad12-4ae022268d0f"},"external_audience_id":"2389…","id":"b6d841b8-e819-50bb-ba55-f9f0c0e679bf","identifiers":["EMAIL","PHONE","FIRST_NAME"],"last_error":"Last error","lookalike":{"country":"DE","enabled":false,"ratio_percent":2},"mode":"CONTINUOUS","name":"Warm-up · Black Friday early access","platform":"META","purpose":"WARM_UP","source":{"kind":"SEGMENT","ref_id":"20df9eb1-f769-5766-b664-2f6b6529022c"},"stats":{"eligible":3112,"last_sync_at":"2026-11-05T15:48:00Z","match_rate":0.61,"matched":1900,"next_sync_at":"2026-11-05T16:48:00Z","removed_total":288,"source_count":5102,"uploaded":3112},"status":"ACTIVE","suppress_on":["UNSUBSCRIBED","SUPPRESSED","ERASED"],"sync_interval_minutes":60,"updated_at":"2026-11-05T15:48:00Z","warm_up":{"days_before_send":10,"frequency_cap_hint":4,"remove_on":"EMAIL_CLICKED","window_days":14}})).unwrap_or_default(), serde_json::from_value::<AdAudience>(serde_json::json!({"account_id":"6dbd6e0c-5762-510c-a3fa-1ad8d54d11fa","consent_basis":"CONSENT","created_at":"2026-10-23T16:00:00Z","created_by":"alex","exclude_source":{"kind":"SEGMENT"},"external_audience_id":"2406…","id":"bbab0796-23f0-59cb-841b-ce85c5b69aae","identifiers":["EMAIL"],"last_error":"Last error","lookalike":{"country":"DE","enabled":false,"ratio_percent":2},"mode":"CONTINUOUS","name":"Warm-up · Webinar invite (LinkedIn)","platform":"LINKEDIN","purpose":"WARM_UP","source":{"kind":"LIST","ref_id":"09f5b2f9-2adc-5edd-a6fd-c20675680671"},"stats":{"eligible":1804,"last_sync_at":"2026-11-05T15:00:00Z","match_rate":0.68,"matched":1226,"next_sync_at":"2026-11-05T16:48:00Z","removed_total":40,"source_count":2604,"uploaded":1804},"status":"ACTIVE","suppress_on":["UNSUBSCRIBED","SUPPRESSED","ERASED"],"sync_interval_minutes":60,"updated_at":"2026-11-05T15:00:00Z","warm_up":{"days_before_send":10,"frequency_cap_hint":4,"remove_on":"EMAIL_CLICKED","window_days":14}})).unwrap_or_default(), serde_json::from_value::<AdAudience>(serde_json::json!({"account_id":"ded95315-6c77-51a0-b8ea-66b76fada21a","consent_basis":"CONSENT","created_at":"2026-10-22T16:00:00Z","created_by":"alex","exclude_source":{"kind":"SEGMENT"},"external_audience_id":"2423…","id":"fa613142-3567-5941-ad4b-02a09f597962","identifiers":["EMAIL"],"last_error":"Last error","lookalike":{"country":"DE","enabled":false,"ratio_percent":2},"mode":"CONTINUOUS","name":"Retarget · pricing visitors 30 d","platform":"META","purpose":"RETARGET","source":{"kind":"SEGMENT","ref_id":"20df9eb1-f769-5766-b664-2f6b6529022c"},"stats":{"eligible":2206,"last_sync_at":"2026-11-05T15:48:00Z","match_rate":0.61,"matched":1346,"next_sync_at":"2026-11-05T16:48:00Z","removed_total":40,"source_count":2410,"uploaded":2206},"status":"ACTIVE","suppress_on":["UNSUBSCRIBED","SUPPRESSED","ERASED"],"sync_interval_minutes":60,"updated_at":"2026-11-05T15:48:00Z","warm_up":{}})).unwrap_or_default()],limit: 25,page: 1,total: 6,
    })
}
