// User-owned controller for handler 'get_task_overview'.

use crate::handlers::get_task_overview::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::PeriodCount;
#[allow(unused_imports)]
use crate::handlers::types::TaskOutcomeRow;
#[allow(unused_imports)]
use crate::handlers::types::TaskOwnerRow;
#[allow(unused_imports)]
use crate::handlers::types::TaskTotals;
#[allow(unused_imports)]
use crate::handlers::types::TaskTypeRow;

#[handler(GetTaskOverviewController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        by_outcome: vec![],
        by_owner: vec![],
        by_type: vec![],
        completed_per_period: Some(vec![]),
        totals: Default::default(),
    })
}
