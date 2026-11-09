// User-owned controller for handler 'get_predictive_content'.

use crate::handlers::get_predictive_content::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::NameCount;
#[allow(unused_imports)]
use crate::handlers::types::VariantRow;

#[handler(GetPredictiveContentController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        click_to_open_rate: Some(1.5),
        clicks_by_variant: Some(vec![]),
        sent_by_variant: Some(vec![]),
        total_sent: 42,
        total_unique_clicks: 42,
        variants: vec![],
    })
}
