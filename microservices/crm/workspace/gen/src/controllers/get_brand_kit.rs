// User-owned controller for handler 'get_brand_kit'.

use crate::handlers::get_brand_kit::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::BrandColors;

#[handler(GetBrandKitController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "body_font": "Body font",
    //   "button_style": "ROUNDED_FILLED",
    //   "colors": {
    //     "accent": "Accent",
    //     "ground": "Ground",
    //     "ink": "Ink",
    //     "positive": "Positive"
    //   },
    //   "heading_font": "Heading font",
    //   "id": "94b293ed-258b-5091-8e79-75d2ac471f25",
    //   "logo_url": "https://northwindtraders.example/pricing",
    //   "social_links": [
    //     "Social links"
    //   ]
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "body_font": "Body font",
  "button_style": "ROUNDED_FILLED",
  "colors": {
    "accent": "Accent",
    "ground": "Ground",
    "ink": "Ink",
    "positive": "Positive"
  },
  "heading_font": "Heading font",
  "id": "94b293ed-258b-5091-8e79-75d2ac471f25",
  "logo_url": "https://northwindtraders.example/pricing",
  "social_links": [
    "Social links"
  ]
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        body_font: Some("Body font".to_string()),button_style: Some("ROUNDED_FILLED".to_string()),colors: Some(serde_json::from_value::<BrandColors>(serde_json::json!({"accent":"Accent","ground":"Ground","ink":"Ink","positive":"Positive"})).unwrap_or_default()),heading_font: Some("Heading font".to_string()),id: "94b293ed-258b-5091-8e79-75d2ac471f25".to_string(),logo_url: Some("https://northwindtraders.example/pricing".to_string()),social_links: Some(vec!["Social links".to_string()]),
    })
}
