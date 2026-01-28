// User-owned controller for handler 'list_vendor_invoices'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_accounts_payable_gen::handlers::list_vendor_invoices::{Request, Response};

#[allow(unused_imports)]
use rerp_accounting_accounts_payable_gen::handlers::types::VendorInvoice;

#[handler(ListVendorInvoicesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "items": [
    //     {
    //       "aging_bucket": "CURRENT",
    //       "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //       "created_at": "2024-01-15T09:00:00Z",
    //       "currency_code": "USD",
    //       "id": "a0050e8400-e29b-41d4-a716-446655440000",
    //       "invoice_id": "a0011e8400-e29b-41d4-a716-446655440001",
    //       "original_amount": rust_decimal::Decimal::new(15000, 0),
    //       "outstanding_amount": rust_decimal::Decimal::new(15000, 0),
    //       "status": "OUTSTANDING",
    //       "updated_at": "2024-01-15T09:00:00Z",
    //       "vendor_id": "411e8400-e29b-41d4-a716-446655440001"
    //     }
    //   ],
    //   "limit": 20,
    //   "page": 1,
    //   "total": 1
    // }

    Response {
        items: Some(vec![serde_json::from_value::<VendorInvoice>(serde_json::json!({"aging_bucket":"CURRENT","company_id":"550e8400-e29b-41d4-a716-446655440000","created_at":"2024-01-15T09:00:00Z","currency_code":"USD","id":"a0050e8400-e29b-41d4-a716-446655440000","invoice_id":"a0011e8400-e29b-41d4-a716-446655440001","original_amount":15000.0,"outstanding_amount":15000.0,"status":"OUTSTANDING","updated_at":"2024-01-15T09:00:00Z","vendor_id":"411e8400-e29b-41d4-a716-446655440001"})).unwrap_or_default()]),limit: Some(20),page: Some(1),total: Some(1),
    }
}
