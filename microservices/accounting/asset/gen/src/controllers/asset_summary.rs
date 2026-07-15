// User-owned controller for handler 'asset_summary'.

use crate::handlers::asset_summary::{Request, Response};
use brrtrouter::typed::HttpJson;
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;

#[handler(AssetSummaryController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> HttpJson<Response> {
    HttpJson::ok(Response {
        as_of_date: Some("example".to_string()),
        by_category: Some(vec![]),
        by_status: Some(Default::default()),
        company_id: "example".to_string(),
        currency_code: Some("example".to_string()),
        depreciation_expense_current_year: Some(3.14),
        disposals_current_year: Some(42),
        total_accumulated_depreciation: 3.14,
        total_acquisition_cost: 3.14,
        total_assets: Some(42),
        total_net_book_value: 3.14,
    })
}
