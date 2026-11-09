// User-owned controller for handler 'get_deal_forecast'.

use crate::handlers::get_deal_forecast::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::ForecastAccuracyEntry;
#[allow(unused_imports)]
use crate::handlers::types::ForecastBasis;
#[allow(unused_imports)]
use crate::handlers::types::ForecastPeriod;

#[handler(GetDealForecastController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        accuracy: Some(vec![]),
        basis: Default::default(),
        currency: Some("example".to_string()),
        generated_at: Some("example".to_string()),
        granularity: "example".to_string(),
        periods: vec![],
        pipeline_id: "example".to_string(),
    })
}
