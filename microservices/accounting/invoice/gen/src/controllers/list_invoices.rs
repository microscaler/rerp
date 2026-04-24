
// User-owned controller for handler 'list_invoices'.

use brrtrouter_macros::handler;
use brrtrouter::typed::TypedHandlerRequest;
use crate::handlers::list_invoices::{ Request, Response };


#[allow(unused_imports)]
use crate::handlers::types::Invoice;



#[handler(ListInvoicesController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    
    // Example response:
        // {
        //   "items": [
        //     {
        //       "company_id": "550e8400-e29b-41d4-a716-446655440000",
        //       "created_at": "2024-01-15T09:00:00Z",
        //       "currency_code": "USD",
        //       "customer_id": "111e8400-e29b-41d4-a716-446655440001",
        //       "discount_amount": 0.0,
        //       "due_date": "2024-02-15",
        //       "exchange_rate": 1.0,
        //       "id": "a0010e8400-e29b-41d4-a716-446655440000",
        //       "invoice_date": "2024-01-15",
        //       "invoice_number": "INV-2024-001",
        //       "invoice_type": "CUSTOMER_INVOICE",
        //       "outstanding_amount": 11000.0,
        //       "paid_amount": 0.0,
        //       "payment_state": "NOT_PAID",
        //       "posted_at": "2024-01-15T10:00:00Z",
        //       "status": "POSTED",
        //       "subtotal": 10000.0,
        //       "tax_amount": 1000.0,
        //       "total_amount": 11000.0,
        //       "updated_at": "2024-01-15T10:00:00Z"
        //     }
        //   ],
        //   "limit": 20,
        //   "page": 1,
        //   "total": 1
        // }
    match serde_json::from_str::<Response>(r###"{
  "items": [
    {
      "company_id": "550e8400-e29b-41d4-a716-446655440000",
      "created_at": "2024-01-15T09:00:00Z",
      "currency_code": "USD",
      "customer_id": "111e8400-e29b-41d4-a716-446655440001",
      "discount_amount": 0.0,
      "due_date": "2024-02-15",
      "exchange_rate": 1.0,
      "id": "a0010e8400-e29b-41d4-a716-446655440000",
      "invoice_date": "2024-01-15",
      "invoice_number": "INV-2024-001",
      "invoice_type": "CUSTOMER_INVOICE",
      "outstanding_amount": 11000.0,
      "paid_amount": 0.0,
      "payment_state": "NOT_PAID",
      "posted_at": "2024-01-15T10:00:00Z",
      "status": "POSTED",
      "subtotal": 10000.0,
      "tax_amount": 1000.0,
      "total_amount": 11000.0,
      "updated_at": "2024-01-15T10:00:00Z"
    }
  ],
  "limit": 20,
  "page": 1,
  "total": 1
}"###) {
        Ok(parsed) => return parsed,
        Err(e) => {
            eprintln!("Failed to parse mock example JSON into Response: {}", e);
            // Fallback to empty default structs below
        }
    }
    
    Response {
        items: Some(vec![serde_json::from_value::<Invoice>(serde_json::json!({"company_id":"550e8400-e29b-41d4-a716-446655440000","created_at":"2024-01-15T09:00:00Z","currency_code":"USD","customer_id":"111e8400-e29b-41d4-a716-446655440001","discount_amount":0.0,"due_date":"2024-02-15","exchange_rate":1.0,"id":"a0010e8400-e29b-41d4-a716-446655440000","invoice_date":"2024-01-15","invoice_number":"INV-2024-001","invoice_type":"CUSTOMER_INVOICE","outstanding_amount":11000.0,"paid_amount":0.0,"payment_state":"NOT_PAID","posted_at":"2024-01-15T10:00:00Z","status":"POSTED","subtotal":10000.0,"tax_amount":1000.0,"total_amount":11000.0,"updated_at":"2024-01-15T10:00:00Z"})).unwrap_or_default()]),limit: Some(20),page: Some(1),total: Some(1),
    }
    
    
}
