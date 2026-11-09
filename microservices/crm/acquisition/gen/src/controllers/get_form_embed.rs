// User-owned controller for handler 'get_form_embed'.

use crate::handlers::get_form_embed::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(GetFormEmbedController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "form_id": "c49b4999-d0ed-58c1-a824-1ffe42b213e8",
    //   "hosted_url": "https://northwindtraders.example/pricing",
    //   "inline_html": "Inline html",
    //   "script_tag": "Script tag"
    // }
    match serde_json::from_str::<Response>(
        r#"{
  "form_id": "c49b4999-d0ed-58c1-a824-1ffe42b213e8",
  "hosted_url": "https://northwindtraders.example/pricing",
  "inline_html": "Inline html",
  "script_tag": "Script tag"
}"#,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        form_id: "c49b4999-d0ed-58c1-a824-1ffe42b213e8".to_string(),
        hosted_url: "https://northwindtraders.example/pricing".to_string(),
        inline_html: Some("Inline html".to_string()),
        script_tag: "Script tag".to_string(),
    })
}
