// User-owned controller for handler 'list_social_posts'.

use crate::handlers::list_social_posts::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::SocialPost;

#[handler(ListSocialPostsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "body": "October release notes are out. Highlights in the thread.",
    //       "created_at": "2026-10-28T16:00:00Z",
    //       "created_by": "alex",
    //       "id": "de4620a8-882f-5f9c-88b0-d57c19355d18",
    //       "media": [],
    //       "requires_approval": false,
    //       "scheduled_at": "2026-11-02T09:30:00+02:00",
    //       "source": "alex",
    //       "status": "PUBLISHED",
    //       "targets": [
    //         {
    //           "account_id": "a41279e1-eaf5-5755-8860-1b6c8aebc4e3",
    //           "error": "Error",
    //           "permalink": "https://x.example/acme/0",
    //           "platform": "X",
    //           "status": "PUBLISHED"
    //         }
    //       ],
    //       "updated_at": "2026-10-28T16:00:00Z"
    //     },
    //     {
    //       "body": "How Northwind cut onboarding time by 40% (case study).",
    //       "created_at": "2026-10-28T16:00:00Z",
    //       "created_by": "alex",
    //       "id": "9502bdab-c9a3-5d31-a3a0-836d1b2265a9",
    //       "media": [],
    //       "requires_approval": false,
    //       "scheduled_at": "2026-11-02T12:00:00+02:00",
    //       "source": "alex",
    //       "status": "PUBLISHED",
    //       "targets": [
    //         {
    //           "account_id": "5bc54618-c69f-51c2-b8c5-adc463b9e422",
    //           "error": "Error",
    //           "permalink": "https://linkedin.example/acme/1",
    //           "platform": "LINKEDIN",
    //           "status": "PUBLISHED"
    //         }
    //       ],
    //       "updated_at": "2026-10-28T16:00:00Z"
    //     },
    //     {
    //       "body": "Pre-market: three charts we are watching this week.",
    //       "created_at": "2026-10-29T16:00:00Z",
    //       "created_by": "alex",
    //       "id": "430fbf7a-0185-5395-8414-939caf4d4c78",
    //       "media": [],
    //       "requires_approval": false,
    //       "scheduled_at": "2026-11-03T09:30:00+02:00",
    //       "source": "alex",
    //       "status": "PUBLISHED",
    //       "targets": [
    //         {
    //           "account_id": "a41279e1-eaf5-5755-8860-1b6c8aebc4e3",
    //           "error": "Error",
    //           "permalink": "https://x.example/acme/2",
    //           "platform": "X",
    //           "status": "PUBLISHED"
    //         }
    //       ],
    //       "updated_at": "2026-10-29T16:00:00Z"
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 10
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "body": "October release notes are out. Highlights in the thread.",
      "created_at": "2026-10-28T16:00:00Z",
      "created_by": "alex",
      "id": "de4620a8-882f-5f9c-88b0-d57c19355d18",
      "media": [],
      "requires_approval": false,
      "scheduled_at": "2026-11-02T09:30:00+02:00",
      "source": "alex",
      "status": "PUBLISHED",
      "targets": [
        {
          "account_id": "a41279e1-eaf5-5755-8860-1b6c8aebc4e3",
          "error": "Error",
          "permalink": "https://x.example/acme/0",
          "platform": "X",
          "status": "PUBLISHED"
        }
      ],
      "updated_at": "2026-10-28T16:00:00Z"
    },
    {
      "body": "How Northwind cut onboarding time by 40% (case study).",
      "created_at": "2026-10-28T16:00:00Z",
      "created_by": "alex",
      "id": "9502bdab-c9a3-5d31-a3a0-836d1b2265a9",
      "media": [],
      "requires_approval": false,
      "scheduled_at": "2026-11-02T12:00:00+02:00",
      "source": "alex",
      "status": "PUBLISHED",
      "targets": [
        {
          "account_id": "5bc54618-c69f-51c2-b8c5-adc463b9e422",
          "error": "Error",
          "permalink": "https://linkedin.example/acme/1",
          "platform": "LINKEDIN",
          "status": "PUBLISHED"
        }
      ],
      "updated_at": "2026-10-28T16:00:00Z"
    },
    {
      "body": "Pre-market: three charts we are watching this week.",
      "created_at": "2026-10-29T16:00:00Z",
      "created_by": "alex",
      "id": "430fbf7a-0185-5395-8414-939caf4d4c78",
      "media": [],
      "requires_approval": false,
      "scheduled_at": "2026-11-03T09:30:00+02:00",
      "source": "alex",
      "status": "PUBLISHED",
      "targets": [
        {
          "account_id": "a41279e1-eaf5-5755-8860-1b6c8aebc4e3",
          "error": "Error",
          "permalink": "https://x.example/acme/2",
          "platform": "X",
          "status": "PUBLISHED"
        }
      ],
      "updated_at": "2026-10-29T16:00:00Z"
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 10
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<SocialPost>(serde_json::json!({"body":"October release notes are out. Highlights in the thread.","created_at":"2026-10-28T16:00:00Z","created_by":"alex","id":"de4620a8-882f-5f9c-88b0-d57c19355d18","media":[],"requires_approval":false,"scheduled_at":"2026-11-02T09:30:00+02:00","source":"alex","status":"PUBLISHED","targets":[{"account_id":"a41279e1-eaf5-5755-8860-1b6c8aebc4e3","error":"Error","permalink":"https://x.example/acme/0","platform":"X","status":"PUBLISHED"}],"updated_at":"2026-10-28T16:00:00Z"})).unwrap_or_default(), serde_json::from_value::<SocialPost>(serde_json::json!({"body":"How Northwind cut onboarding time by 40% (case study).","created_at":"2026-10-28T16:00:00Z","created_by":"alex","id":"9502bdab-c9a3-5d31-a3a0-836d1b2265a9","media":[],"requires_approval":false,"scheduled_at":"2026-11-02T12:00:00+02:00","source":"alex","status":"PUBLISHED","targets":[{"account_id":"5bc54618-c69f-51c2-b8c5-adc463b9e422","error":"Error","permalink":"https://linkedin.example/acme/1","platform":"LINKEDIN","status":"PUBLISHED"}],"updated_at":"2026-10-28T16:00:00Z"})).unwrap_or_default(), serde_json::from_value::<SocialPost>(serde_json::json!({"body":"Pre-market: three charts we are watching this week.","created_at":"2026-10-29T16:00:00Z","created_by":"alex","id":"430fbf7a-0185-5395-8414-939caf4d4c78","media":[],"requires_approval":false,"scheduled_at":"2026-11-03T09:30:00+02:00","source":"alex","status":"PUBLISHED","targets":[{"account_id":"a41279e1-eaf5-5755-8860-1b6c8aebc4e3","error":"Error","permalink":"https://x.example/acme/2","platform":"X","status":"PUBLISHED"}],"updated_at":"2026-10-29T16:00:00Z"})).unwrap_or_default()],limit: 25,page: 1,total: 10,
    })
}
