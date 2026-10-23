// BRRTRouter: user-owned
//! Customers, by name (ADR 003: every one of them crossed in through a conversion).

use brrtrouter::typed::{HttpJson, TypedHandlerRequest};
use brrtrouter_macros::handler;
use rerp_crm_accounts_gen::handlers::list_accounts::Request;
use serde_json::Value;

use crate::http_support::{fail, with_caller};

#[handler(ListAccountsController)]
pub fn handle(req: TypedHandlerRequest<Request>) -> HttpJson<Value> {
    if let Err(denied) = crate::auth::require_viewer(req.jwt_claims.as_ref()) {
        return denied;
    }
    let data = &req.data;
    let page = data.page.unwrap_or(1);
    let limit = data.limit.unwrap_or(20);
    if page < 1 || !(1..=100).contains(&limit) {
        return fail(400, "page must be at least 1 and limit 1..=100");
    }
    if data.filter_parent_id.is_some() || data.filter_industry_id.is_some() {
        return fail(400, "parent and industry filters are not supported yet");
    }
    let search = data
        .search
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());
    match with_caller(req.jwt_claims.as_ref(), |executor| {
        crate::accounts::list(
            executor,
            search,
            data.filter_active,
            page as u32,
            limit as u32,
        )
    }) {
        Ok(page) => HttpJson::new(200, page),
        Err(response) => response,
    }
}
