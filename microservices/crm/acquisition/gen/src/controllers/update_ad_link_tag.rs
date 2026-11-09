// User-owned controller for handler 'update_ad_link_tag'.

use crate::handlers::update_ad_link_tag::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(UpdateAdLinkTagController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "add_click_id": true,
    //   "add_contact_token": true,
    //   "applies_to": "CAMPAIGN_EMAIL",
    //   "domains": [
    //     "Domains"
    //   ],
    //   "enabled": true,
    //   "id": "5d10bed4-1f7c-5c35-bd68-3e84ba31aa08",
    //   "utm_campaign": "{campaign.slug}",
    //   "utm_content": "Utm content",
    //   "utm_medium": "Utm medium",
    //   "utm_source": "email"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "add_click_id": true,
  "add_contact_token": true,
  "applies_to": "CAMPAIGN_EMAIL",
  "domains": [
    "Domains"
  ],
  "enabled": true,
  "id": "5d10bed4-1f7c-5c35-bd68-3e84ba31aa08",
  "utm_campaign": "{campaign.slug}",
  "utm_content": "Utm content",
  "utm_medium": "Utm medium",
  "utm_source": "email"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        add_click_id: Some(true),
        add_contact_token: Some(true),
        applies_to: "CAMPAIGN_EMAIL".to_string(),
        domains: Some(vec!["Domains".to_string()]),
        enabled: Some(true),
        id: Some("5d10bed4-1f7c-5c35-bd68-3e84ba31aa08".to_string()),
        utm_campaign: Some("{campaign.slug}".to_string()),
        utm_content: Some("Utm content".to_string()),
        utm_medium: Some("Utm medium".to_string()),
        utm_source: Some("email".to_string()),
    })
}
