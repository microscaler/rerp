// User-owned controller for handler 'render_template'.

use crate::handlers::render_template::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(RenderTemplateController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    // Example response:
    // {
    //   "from_address": "hello@acme.example",
    //   "from_name": "Acme Co",
    //   "html": "<p>Hi Priya,</p><p>A week ago you signed up. Here is what you have set up so far at Northwind Traders, and three things worth doing next.</p><ul><li>Connect your data source</li><li>Invite a teammate</li><li>Turn on the weekly digest</li></ul><p><a href=\"#\">Open your workspace</a></p>",
    //   "preheader": "A week in, and three things worth doing next",
    //   "subject": "Day 7: what you have set up so far",
    //   "text": "Hi Priya,\n\nA week ago you signed up…",
    //   "unresolved": []
    // }
    match serde_json::from_str::<Response>(
        r##"{
  "from_address": "hello@acme.example",
  "from_name": "Acme Co",
  "html": "<p>Hi Priya,</p><p>A week ago you signed up. Here is what you have set up so far at Northwind Traders, and three things worth doing next.</p><ul><li>Connect your data source</li><li>Invite a teammate</li><li>Turn on the weekly digest</li></ul><p><a href=\"#\">Open your workspace</a></p>",
  "preheader": "A week in, and three things worth doing next",
  "subject": "Day 7: what you have set up so far",
  "text": "Hi Priya,\n\nA week ago you signed up…",
  "unresolved": []
}"##,
    ) {
        Ok(parsed) => return HttpJson::ok(parsed),
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }

    HttpJson::ok(Response {
        from_address: Some("hello@acme.example".to_string()),from_name: Some("Acme Co".to_string()),html: "<p>Hi Priya,</p><p>A week ago you signed up. Here is what you have set up so far at Northwind Traders, and three things worth doing next.</p><ul><li>Connect your data source</li><li>Invite a teammate</li><li>Turn on the weekly digest</li></ul><p><a href=\"#\">Open your workspace</a></p>".to_string(),preheader: Some("A week in, and three things worth doing next".to_string()),subject: Some("Day 7: what you have set up so far".to_string()),text: "Hi Priya,\n\nA week ago you signed up…".to_string(),unresolved: Some(vec![]),
    })
}
