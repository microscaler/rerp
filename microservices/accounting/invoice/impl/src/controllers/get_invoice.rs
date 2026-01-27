// User-owned controller for handler 'get_invoice'.
use brrtrouter::typed::TypedHandlerRequest;
use brrtrouter_macros::handler;
use rerp_accounting_invoice_gen::handlers::get_invoice::{Request, Response};

#[handler(GetInvoiceController)]
pub fn handle(_req: TypedHandlerRequest<Request>) -> Response {
    // Example response:
    // {
    //   "company_id": "550e8400-e29b-41d4-a716-446655440000",
    //   "created_at": "2024-01-15T09:00:00Z",
    //   "currency_code": "USD",
    //   "customer_id": "111e8400-e29b-41d4-a716-446655440001",
    //   "discount_amount": 0.0,
    //   "due_date": "2024-02-15",
    //   "exchange_rate": 1.0,
    //   "id": "a0010e8400-e29b-41d4-a716-446655440000",
    //   "invoice_date": "2024-01-15",
    //   "invoice_number": "INV-2024-001",
    //   "invoice_type": "CUSTOMER_INVOICE",
    //   "outstanding_amount": 11000.0,
    //   "paid_amount": 0.0,
    //   "payment_state": "NOT_PAID",
    //   "posted_at": "2024-01-15T10:00:00Z",
    //   "status": "POSTED",
    //   "subtotal": 10000.0,
    //   "tax_amount": 1000.0,
    //   "total_amount": 11000.0,
    //   "updated_at": "2024-01-15T10:00:00Z"
    // }

    Response {
        cancelled_at: Some("example".to_string()),
        company_id: Some("550e8400-e29b-41d4-a716-446655440000".to_string()),
        created_at: Some("2024-01-15T09:00:00Z".to_string()),
        currency_code: "USD".to_string(),
        customer_id: Some("111e8400-e29b-41d4-a716-446655440001".to_string()),
        discount_amount: Some(0.0),
        due_date: Some("2024-02-15".to_string()),
        exchange_rate: Some(1.0),
        id: "a0010e8400-e29b-41d4-a716-446655440000".to_string(),
        internal_notes: Some("example".to_string()),
        invoice_date: "2024-01-15".to_string(),
        invoice_number: "INV-2024-001".to_string(),
        invoice_type: "CUSTOMER_INVOICE".to_string(),
        metadata: Some(Default::default()),
        notes: Some("example".to_string()),
        outstanding_amount: Some(11000.0),
        paid_amount: Some(0.0),
        paid_at: Some("example".to_string()),
        payment_state: "NOT_PAID".to_string(),
        payment_term_id: Some("example".to_string()),
        posted_at: Some("2024-01-15T10:00:00Z".to_string()),
        reference_number: Some("example".to_string()),
        status: "POSTED".to_string(),
        subtotal: Some(10000.0),
        tax_amount: Some(1000.0),
        total_amount: Some(11000.0),
        updated_at: Some("2024-01-15T10:00:00Z".to_string()),
        vendor_id: Some("example".to_string()),
        vendor_reference: Some("example".to_string()),
    }
}
