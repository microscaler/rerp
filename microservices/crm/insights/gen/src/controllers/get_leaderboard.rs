// User-owned controller for handler 'get_leaderboard'.

use crate::handlers::get_leaderboard::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::LeaderboardEntry;

#[handler(GetLeaderboardController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "rank": 3,
    //       "user_id": "alex",
    //       "user_name": "User name",
    //       "value": 12.5
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 2
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "rank": 3,
      "user_id": "alex",
      "user_name": "User name",
      "value": 12.5
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 2
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<LeaderboardEntry>(
            serde_json::json!({"rank":3,"user_id":"alex","user_name":"User name","value":12.5}),
        )
        .unwrap_or_default()],
        limit: 25,
        page: 1,
        total: 2,
    })
}
