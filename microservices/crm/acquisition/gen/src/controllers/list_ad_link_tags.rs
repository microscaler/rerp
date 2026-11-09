// User-owned controller for handler 'list_ad_link_tags'.

use crate::handlers::list_ad_link_tags::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::AdLinkTagRule;

#[handler(ListAdLinkTagsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "applies_to": "CAMPAIGN_EMAIL"
    //     }
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r###"{
  "items": [
    {
      "applies_to": "CAMPAIGN_EMAIL"
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

    HttpJson::ok(Response(vec![serde_json::from_value::<AdLinkTagRule>(
        serde_json::json!({"applies_to":"CAMPAIGN_EMAIL"}),
    )
    .unwrap_or_default()]))
}
