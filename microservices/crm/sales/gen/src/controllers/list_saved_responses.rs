// User-owned controller for handler 'list_saved_responses'.

use crate::handlers::list_saved_responses::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::SavedResponse;

#[handler(ListSavedResponsesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "body": "Hi %FIRSTNAME%,\n\nYou're on day %TRIAL_DAY% of your trial of %PRODUCT%. I noticed %ACCOUNT% hasn't set up %FEATURE% yet — happy to walk you through it in 15 minutes.\n\n%SIGNATURE%",
    //       "category": "Sales",
    //       "category_id": "113ca5aa-e3f2-576a-b6b5-79dbb715308e",
    //       "id": "9587a888-ee0e-50ac-8e1a-38a273fccd36",
    //       "name": "Northwind Traders",
    //       "subject": "How is your %FIRSTNAME% trial going?",
    //       "title": "Trial follow-up",
    //       "updated_at": "2026-11-04T16:00:00Z",
    //       "use_count": 41
    //     },
    //     {
    //       "body": "Hi %FIRSTNAME%,\n\nAttached is the proposal we discussed…\n\n%SIGNATURE%",
    //       "category": "Sales",
    //       "category_id": "113ca5aa-e3f2-576a-b6b5-79dbb715308e",
    //       "id": "eac7c2fa-4409-5886-b674-5a0d0ed8ddb6",
    //       "name": "Northwind Traders",
    //       "subject": "Proposal for %ACCOUNT%",
    //       "title": "Proposal cover note",
    //       "updated_at": "2026-10-27T16:00:00Z",
    //       "use_count": 18
    //     },
    //     {
    //       "body": "Hi %FIRSTNAME%, see you on %MEETING_DATE%.\n\n%SIGNATURE%",
    //       "category": "Scheduling",
    //       "category_id": "cab98488-f673-5409-9af6-81de2f44a796",
    //       "id": "5bad7783-ebef-5909-8e69-53319d7f4cfd",
    //       "name": "Northwind Traders",
    //       "subject": "Confirmed: %MEETING_DATE%",
    //       "title": "Meeting confirmation",
    //       "updated_at": "2026-10-16T16:00:00Z",
    //       "use_count": 66
    //     }
    //   ],
    //   "limit": 25,
    //   "page": 1,
    //   "total": 4
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "items": [
    {
      "body": "Hi %FIRSTNAME%,\n\nYou're on day %TRIAL_DAY% of your trial of %PRODUCT%. I noticed %ACCOUNT% hasn't set up %FEATURE% yet — happy to walk you through it in 15 minutes.\n\n%SIGNATURE%",
      "category": "Sales",
      "category_id": "113ca5aa-e3f2-576a-b6b5-79dbb715308e",
      "id": "9587a888-ee0e-50ac-8e1a-38a273fccd36",
      "name": "Northwind Traders",
      "subject": "How is your %FIRSTNAME% trial going?",
      "title": "Trial follow-up",
      "updated_at": "2026-11-04T16:00:00Z",
      "use_count": 41
    },
    {
      "body": "Hi %FIRSTNAME%,\n\nAttached is the proposal we discussed…\n\n%SIGNATURE%",
      "category": "Sales",
      "category_id": "113ca5aa-e3f2-576a-b6b5-79dbb715308e",
      "id": "eac7c2fa-4409-5886-b674-5a0d0ed8ddb6",
      "name": "Northwind Traders",
      "subject": "Proposal for %ACCOUNT%",
      "title": "Proposal cover note",
      "updated_at": "2026-10-27T16:00:00Z",
      "use_count": 18
    },
    {
      "body": "Hi %FIRSTNAME%, see you on %MEETING_DATE%.\n\n%SIGNATURE%",
      "category": "Scheduling",
      "category_id": "cab98488-f673-5409-9af6-81de2f44a796",
      "id": "5bad7783-ebef-5909-8e69-53319d7f4cfd",
      "name": "Northwind Traders",
      "subject": "Confirmed: %MEETING_DATE%",
      "title": "Meeting confirmation",
      "updated_at": "2026-10-16T16:00:00Z",
      "use_count": 66
    }
  ],
  "limit": 25,
  "page": 1,
  "total": 4
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        items: vec![serde_json::from_value::<SavedResponse>(serde_json::json!({"body":"Hi %FIRSTNAME%,\n\nYou're on day %TRIAL_DAY% of your trial of %PRODUCT%. I noticed %ACCOUNT% hasn't set up %FEATURE% yet — happy to walk you through it in 15 minutes.\n\n%SIGNATURE%","category":"Sales","category_id":"113ca5aa-e3f2-576a-b6b5-79dbb715308e","id":"9587a888-ee0e-50ac-8e1a-38a273fccd36","name":"Northwind Traders","subject":"How is your %FIRSTNAME% trial going?","title":"Trial follow-up","updated_at":"2026-11-04T16:00:00Z","use_count":41})).unwrap_or_default(), serde_json::from_value::<SavedResponse>(serde_json::json!({"body":"Hi %FIRSTNAME%,\n\nAttached is the proposal we discussed…\n\n%SIGNATURE%","category":"Sales","category_id":"113ca5aa-e3f2-576a-b6b5-79dbb715308e","id":"eac7c2fa-4409-5886-b674-5a0d0ed8ddb6","name":"Northwind Traders","subject":"Proposal for %ACCOUNT%","title":"Proposal cover note","updated_at":"2026-10-27T16:00:00Z","use_count":18})).unwrap_or_default(), serde_json::from_value::<SavedResponse>(serde_json::json!({"body":"Hi %FIRSTNAME%, see you on %MEETING_DATE%.\n\n%SIGNATURE%","category":"Scheduling","category_id":"cab98488-f673-5409-9af6-81de2f44a796","id":"5bad7783-ebef-5909-8e69-53319d7f4cfd","name":"Northwind Traders","subject":"Confirmed: %MEETING_DATE%","title":"Meeting confirmation","updated_at":"2026-10-16T16:00:00Z","use_count":66})).unwrap_or_default()],limit: 25,page: 1,total: 4,
    })
}
