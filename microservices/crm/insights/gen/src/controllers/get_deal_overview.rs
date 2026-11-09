// User-owned controller for handler 'get_deal_overview'.

use crate::handlers::get_deal_overview::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::DealOwnerRow;
#[allow(unused_imports)]
use crate::handlers::types::DealStageRow;
#[allow(unused_imports)]
use crate::handlers::types::DealTotals;
#[allow(unused_imports)]
use crate::handlers::types::DealValueBracket;
#[allow(unused_imports)]
use crate::handlers::types::PeriodCount;

#[handler(GetDealOverviewController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        by_owner: vec![],
        by_stage: vec![],
        created_per_period: Some(vec![]),
        currency: "example".to_string(),
        totals: Default::default(),
        value_brackets: vec![],
    })
}
