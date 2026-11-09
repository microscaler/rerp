// User-owned controller for handler 'get_job'.

use crate::handlers::get_job::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::JobEntity;
#[allow(unused_imports)]
use crate::handlers::types::JobKind;
#[allow(unused_imports)]
use crate::handlers::types::JobOptions;
#[allow(unused_imports)]
use crate::handlers::types::JobStatus;

#[handler(GetJobController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        created_rows: Some(42),
        entity: Default::default(),
        error_rows: Some(42),
        file_name: Some("example".to_string()),
        finished_at: Some("example".to_string()),
        id: "example".to_string(),
        integration_id: Some("example".to_string()),
        kind: Default::default(),
        message: Some("example".to_string()),
        options: Some(Default::default()),
        processed_rows: Some(42),
        progress_pct: Some(1.5),
        queued_at: "example".to_string(),
        requested_by: "example".to_string(),
        result_expires_at: Some("example".to_string()),
        result_url: Some("example".to_string()),
        skipped_rows: Some(42),
        source: Some("example".to_string()),
        started_at: Some("example".to_string()),
        status: Default::default(),
        total_rows: Some(42),
        updated_rows: Some(42),
    })
}
