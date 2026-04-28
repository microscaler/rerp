// User-owned controller for handler 'trigger_statement_sync'.

use crate::handlers::trigger_statement_sync::{Request, Response};
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(TriggerStatementSyncController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    Response {
        statements_found: Some(42),
        sync_status: Some("example".to_string()),
    }
}
