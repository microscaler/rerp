// User-owned controller for handler 'get_deal_suggestions'.

use crate::handlers::get_deal_suggestions::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DealSuggestion;

#[handler(GetDealSuggestionsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "action": "Draft follow-up email",
    //       "href": "/crm/mailboxes?compose=af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
    //       "kind": "FOLLOW_UP",
    //       "text": "Deal has been in Proposal 3 days; similar deals close within 12 days of a proposal when a follow-up call is booked."
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "action": "Draft follow-up email",
      "href": "/crm/mailboxes?compose=af512c4f-0beb-50b6-bf61-7a9d8bd417b5",
      "kind": "FOLLOW_UP",
      "text": "Deal has been in Proposal 3 days; similar deals close within 12 days of a proposal when a follow-up call is booked."
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

    HttpJson::ok(Response(vec![serde_json::from_value::<DealSuggestion>(serde_json::json!({"action":"Draft follow-up email","href":"/crm/mailboxes?compose=af512c4f-0beb-50b6-bf61-7a9d8bd417b5","kind":"FOLLOW_UP","text":"Deal has been in Proposal 3 days; similar deals close within 12 days of a proposal when a follow-up call is booked."})).unwrap_or_default()]))
}
