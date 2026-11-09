// User-owned controller for handler 'public_preferences_get'.

use crate::handlers::public_preferences_get::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(PublicPreferencesGetController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "email_masked": "a***e@example.com",
    //   "lists": [
    //     {
    //       "list_id": "5d4df9aa-18c7-58a5-b87a-7af2ce28c75f",
    //       "name": "Northwind Traders",
    //       "subscribed": false
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "email_masked": "a***e@example.com",
  "lists": [
    {
      "list_id": "5d4df9aa-18c7-58a5-b87a-7af2ce28c75f",
      "name": "Northwind Traders",
      "subscribed": false
    }
  ]
}"###,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        email_masked: "a***e@example.com".to_string(),lists: vec![serde_json::from_str::<serde_json::Value>(r#"{"list_id":"5d4df9aa-18c7-58a5-b87a-7af2ce28c75f","name":"Northwind Traders","subscribed":false}"#).unwrap_or(serde_json::Value::Null)],
    })
}
