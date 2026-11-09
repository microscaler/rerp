// User-owned controller for handler 'task_overview_report'.

use crate::handlers::task_overview_report::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(TaskOverviewReportController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        by_outcome: Some(Default::default()),
        by_owner: Some(vec![]),
        by_sentiment: Some(Default::default()),
        by_status: Some(Default::default()),
        by_type: Some(Default::default()),
        completed: Some(42),
        open: Some(42),
        overdue: Some(42),
        series: Some(vec![]),
        total: Some(42),
    })
}
