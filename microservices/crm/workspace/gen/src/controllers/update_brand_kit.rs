// User-owned controller for handler 'update_brand_kit'.

use crate::handlers::update_brand_kit::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::BrandColors;

#[handler(UpdateBrandKitController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "body_font": "Inter",
    //   "button_style": "ROUNDED_FILLED",
    //   "colors": {
    //     "accent": "#1F5AF6",
    //     "ground": "#F6F7F9",
    //     "ink": "#0F1420",
    //     "positive": "#0B7A4B"
    //   },
    //   "heading_font": "Inter",
    //   "id": "272a2106-4955-5cc1-b44c-8e6efc933b02",
    //   "logo_url": "https://northwindtraders.example/pricing",
    //   "social_links": [
    //     "x.com/acme",
    //     "linkedin.com/company/acme"
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r##"{
  "body_font": "Inter",
  "button_style": "ROUNDED_FILLED",
  "colors": {
    "accent": "#1F5AF6",
    "ground": "#F6F7F9",
    "ink": "#0F1420",
    "positive": "#0B7A4B"
  },
  "heading_font": "Inter",
  "id": "272a2106-4955-5cc1-b44c-8e6efc933b02",
  "logo_url": "https://northwindtraders.example/pricing",
  "social_links": [
    "x.com/acme",
    "linkedin.com/company/acme"
  ]
}"##,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        body_font: Some("Inter".to_string()),button_style: Some("ROUNDED_FILLED".to_string()),colors: Some(serde_json::from_value::<BrandColors>(serde_json::json!({"accent":"#1F5AF6","ground":"#F6F7F9","ink":"#0F1420","positive":"#0B7A4B"})).unwrap_or_default()),heading_font: Some("Inter".to_string()),id: "272a2106-4955-5cc1-b44c-8e6efc933b02".to_string(),logo_url: Some("https://northwindtraders.example/pricing".to_string()),social_links: Some(vec!["x.com/acme".to_string(), "linkedin.com/company/acme".to_string()]),
    })
}
