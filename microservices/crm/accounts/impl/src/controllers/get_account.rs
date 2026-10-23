// BRRTRouter: user-owned
//! One customer, with how it crossed into RERP.

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_accounts_gen::handlers::get_account::Request;
use serde_json::Value;
use uuid::Uuid;

use crate::http_support::{fail, with_caller};

#[handler(GetAccountController)]
pub fn handle(req: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    if let Err(denied) = crate::auth::require_viewer(req.jwt_claims.as_ref()) {
        return denied;
    }
    let Ok(id) = Uuid::parse_str(&req.data.id) else {
        return fail(400, "id must be a UUID");
    };
    match with_caller(req.jwt_claims.as_ref(), |executor| {
        crate::accounts::get(executor, id)
    }) {
        Ok(Some(account)) => HttpJson::new(200, account),
        Ok(None) => fail(404, "customer not found"),
        Err(response) => response,
    }
}
