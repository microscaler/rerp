// User-owned controller for handler 'get_email_client_trends'.

use crate::handlers::get_email_client_trends::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[allow(unused_imports)]
use crate::handlers::types::EmailClientRow;
#[allow(unused_imports)]
use crate::handlers::types::NameCount;

#[handler(GetEmailClientTrendsController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        by_family: Some(vec![]),
        clients: vec![],
        total_opens: 42,
    })
}
