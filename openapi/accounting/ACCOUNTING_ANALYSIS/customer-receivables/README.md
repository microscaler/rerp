# Customer Invoice & Receivables

> **Component:** Revenue collection lifecycle — customer invoices, payment processing, credit management, aging, dunning, and collections
> **Priority:** P0 — Revenue collection is the lifeblood of any business
> **Odoo Reference:** account.invoice (3,000+ lines), account.payment (1,500 lines), res.partner (customer-specific)

---

## The Pitch

**Buyer Question:** *Can I invoice customers automatically, accept payments online, track what's owed in real-time, and collect overdue amounts efficiently — all without manual reconciliation?*\

Accounts receivable is where revenue becomes cash. If you can't track what customers owe, when it's due, and who to contact when it's late, you're bleeding working capital. This component handles the full AR lifecycle from customer onboarding to cash collection, including automated invoicing from orders/subscriptions, online payment acceptance, payment reconciliation, aging analysis, credit limit management, and dunning/collections workflows.

---

## What This Component Does

Accounts Receivable is the controlled inflow of cash. It handles:

1. **Customer Master** — Complete customer directory with credit limits, payment terms, and contact info
2. **Invoice Generation** — Manual, recurring, subscription-based, or auto-generated from sales orders
3. **Online Payments** — Accept payments via integrated gateways (Stripe, PayPal, etc.)
4. **Payment Reconciliation** — Match incoming payments to invoices automatically
5. **Credit Management** — Set and enforce credit limits per customer
6. **Dunning & Collections** — Automated reminders, follow-up emails, escalation to collections
7. **Aging Analysis** — Track receivables aging (current, 30, 60, 90, 120+ days)
8. **Bad Debt Management** — Write-off uncollectible accounts

---

## Entity Model

### Customer Invoice Entity

The core revenue document sent to a customer:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Invoice reference number (auto-generated, e.g., "INV/2025/0001") |
| `customer_invoice_number` | String (128) | Yes | Original customer invoice number |
| `partner_id` | Foreign Key: Partner | Yes | Customer (bill-to) |
| `commercial_partner_id` | Foreign Key: Partner | Yes | Customer (commercial entity) |
| `invoice_user_id` | Foreign Key: User | No | Salesperson/owner |
| `journal_id` | Foreign Key: Journal | Yes | Revenue journal |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `currency_id` | Foreign Key: Currency | Yes | Invoice currency |
| `invoice_date` | Date | Yes | Invoice date |
| `invoice_date_due` | Date | No | Payment due date (from payment terms) |
| `invoice_date_today` | Date | Computed | Today's date (for aging) |
| `amount_untaxed` | Decimal (15,2) | Yes | Subtotal before tax |
| `amount_tax` | Decimal (15,2) | Yes | Total tax amount |
| `amount_total` | Decimal (15,2) | Yes | Grand total (untaxed + tax) |
| `amount_total_in_currency` | Decimal (15,2) | Computed | Total in invoice currency |
| `amount_paid` | Decimal (15,2) | Computed | Total amount already paid |
| `amount_residual` | Decimal (15,2) | Computed | Remaining balance |
| `amount_paid_signed` | Decimal (15,2) | Computed | Paid amount with sign |
| `amount_residual_signed` | Decimal (15,2) | Computed | Residual with sign |
| `state` | Enum: [DRAFT, VALIDATED, POSTED, PAID, CANCELLED, REVERSED] | Yes | Invoice lifecycle |
| `move_id` | Foreign Key: Move | Computed | Associated journal entry |
| `move_type` | Enum: [OUT Invoice, DEBIT_NOTE, CREDIT_NOTE, RETURN] | Yes | Invoice type |
| `payment_reference` | String (255) | No | Payment reference |
| `ref` | String (255) | No | Internal reference |
| `narration` | Text | No | Invoice description |
| `account_payment_term_id` | Foreign Key: Payment Term | No | Payment terms |
| `sale_order_id` | Foreign Key: Sales Order | No | Linked sales order |
| `subscription_id` | Foreign Key: Subscription | No | Linked subscription (recurring) |
| `payment_status` | Enum: [UNPAID, PARTIALLY_PAID, IN_PAYMENT, FULLY_PAID] | Computed | Derived status |
| `is_tax_recoverable` | Boolean | No | Is tax recoverable? |
| `tax_cash_basis` | Boolean | No | Tax cash basis reporting |
| `created_sales_order_id` | Foreign Key: Sales Order | No | Source sales order |
| `reconciled` | Boolean | Computed | True if fully reconciled |
| `reconciled_percentage` | Float (0-100) | Computed | Payment coverage % |
| `create_uid` | Foreign Key: User | Computed | Creator |
| `create_date` | DateTime | Computed | Created timestamp |
| `write_uid` | Foreign Key: User | Computed | Last modifier |
| `write_date` | DateTime | Computed | Last modified |

**Total fields: ~42.**

### Invoice Line Entity

Individual line items on a customer invoice:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `move_id` | Foreign Key: Move | Yes | Parent invoice |
| `sequence` | Integer | Yes | Display order |
| `product_id` | Foreign Key: Product | No | Product/service |
| `name` | String (255) | No | Line description |
| `quantity` | Float | Yes | Quantity |
| `discount` | Decimal (5,2) | No | Discount % |
| `price_unit` | Decimal (15,2) | Yes | Unit price |
| `price_subtotal` | Decimal (15,2) | Computed | subtotal |
| `price_total` | Decimal (15,2) | Computed | total with tax |
| `tax_ids` | Many2Many: Tax | No | Taxes applied |
| `account_id` | Foreign Key: Account | Yes | Revenue account |
| `analytic_distribution` | JSON | No | Cost center breakdown |
| `product_uom_id` | Foreign Key: UOM | No | Unit of measure |
| `move_line_id` | Foreign Key: Move Line | No | Linked GL line |

**Total fields: ~15.**

### Customer Payment Entity

Payment received from a customer:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `payment_date` | Date | Yes | Payment date |
| `amount` | Decimal (15,2) | Yes | Payment amount |
| `currency_id` | Foreign Key: Currency | Yes | Payment currency |
| `partner_id` | Foreign Key: Partner | Yes | Customer payer |
| `journal_id` | Foreign Key: Journal | Yes | Payment journal (Bank/Cash) |
| `company_id` | Foreign Key: Company | Yes | Company receiving payment |
| `payment_method_id` | Foreign Key: Method | Yes | Received via (wire, card, etc.) |
| `payment_reference` | String (255) | Yes | Payment reference |
| `bank_transaction_id` | Foreign Key: Transaction | No | Linked bank transaction |
| `move_id` | Foreign Key: Move | Computed | Associated GL entry |
| `state` | Enum: [DRAFT, CONFIRMED, PAID, CANCELLED] | Yes | Payment lifecycle |
| `invoice_ids` | Many2Many: Invoice | Yes | Invoices being paid |
| `payment_type` | Enum: [INBOUND, OUTBOUND, TRANSFER] | Yes | Payment direction |
| `communication` | String (255) | No | Payment communication |
| `create_uid` | Foreign Key: User | Computed | Who created |
| `create_date` | DateTime | Computed | When created |

**Total fields: ~17.**

### Customer Credit Limit Entity

Track and enforce customer credit:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `partner_id` | Foreign Key: Partner | Yes | Customer |
| `credit_limit` | Decimal (15,2) | Yes | Maximum allowed exposure |
| `credit_used` | Decimal (15,2) | Computed | Current outstanding balance |
| `credit_available` | Decimal (15,2) | Computed | credit_limit - credit_used |
| `overdue_limit` | Decimal (15,2) | No | Allowance for overdue amounts |
| `credit_hold` | Boolean | No | On credit hold? |
| `credit_hold_reason` | String (255) | No | Reason for hold |
| `company_id` | Foreign Key: Company | Yes | Company |

**Total fields: ~8.**

### Dunning Rule Entity

Automate collections workflows:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Rule name (e.g., "First Reminder", "Second Reminder") |
| `sequence` | Integer | Yes | Display order |
| `delay_after_due` | Integer | Yes | Days after due date to trigger |
| `dunning_level` | Integer | Yes | Dunning level (1, 2, 3...) |
| `subject_template` | String (255) | Yes | Email subject template |
| `body_template` | Text | Yes | Email body template |
| `send_email` | Boolean | Yes | Send email? |
| `create_reminder` | Boolean | No | Create internal reminder? |
| `action` | Enum: [EMAIL, LETTER, CALL, LEGAL, NONE] | No | Follow-up action |
| `invoice_ids` | One2Many: Invoice | Computed | Invoices matched to this rule |

**Total fields: ~11.**

---

## Entity Relationships

```
account.invoice (customer invoices)
  ├── res.partner (partner_id)           ← Customer (bill-to)
  ├── account.journal (journal_id)       ← Revenue journal
  ├── account.move (move_id)             ← GL entry
  ├── account.payment.term (term)        ← Payment terms
  ├── account.invoice.line (line_ids)    ← Line items
  └── account.payment (payment_ids)      ← Linked payments

account.invoice.line
  ├── product.product (product_id)      ← Product/service
  ├── account.account (account_id)       ← Revenue account
  └── account.move.line (move_line_id)   ← GL line

account.payment (customer payments)
  ├── res.partner (partner_id)           ← Customer
  ├── account.journal (journal_id)       ← Payment journal
  ├── account.move (move_id)             ← GL entry
  └── account.invoice (invoice_ids)      ← Invoices being paid (reconciliation)

customer.credit.limit
  └── res.partner (partner_id)           ← Customer

dunning.rule
  └── account.invoice (matched invoices) ← Invoices matching delay conditions
```

---

## Required API Endpoints

### Customer Invoice CRUD

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/invoices` | List customer invoices with filters |
| `GET` | `/invoices/{id}` | Get invoice detail with lines |
| `POST` | `/invoices` | Create draft invoice |
| `PATCH` | `/invoices/{id}` | Update draft invoice |
| `POST` | `/invoices/{id}/validate` | Validate invoice |
| `POST` | `/invoices/{id}/post` | Post invoice (create GL entry) |
| `POST` | `/invoices/{id}/cancel` | Cancel invoice |
| `POST` | `/invoices/batch` | Create batch invoices |
| `POST` | `/invoices/from-sales-order/{id}` | Create invoice from sales order |
| `POST` | `/invoices/from-subscription/{id}` | Create invoice from subscription |

### Online Payments

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/invoices/{id}/pay` | Process online payment for invoice |
| `POST` | `/invoices/pay-all` | Pay all selected invoices |
| `POST` | `/payments/online` | Receive payment via gateway |
| `GET` | `/payments/{id}/status` | Check payment status |
| `POST` | `/payments/{id}/capture` | Capture authorized payment |
| `POST` | `/payments/{id}/refund` | Issue payment refund |

### Payment Reconciliation

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/payments/{id}/reconcile` | Register payment against invoice(s) |
| `POST` | `/invoices/{id}/partial-pay` | Make partial payment |
| `POST` | `/payments/auto-reconcile` | Auto-match payments to invoices |
| `GET` | `/payments/unreconciled` | List unreconciled payments |
| `POST` | `/payments/{id}/unreconcile` | Unreconcile a payment |

### Credit Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/customers/credit` | List all customer credit limits |
| `PATCH` | `/customers/credit/{partner_id}` | Update credit limit |
| `GET` | `/customers/{id}/credit-summary` | Customer credit summary |
| `POST` | `/customers/check-credit` | Check if order exceeds credit limit |

### Dunning & Collections

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/dunning/levels` | List dunning levels |
| `POST` | `/dunning/levels` | Create dunning rule |
| `PATCH` | `/dunning/levels/{id}` | Update rule |
| `POST` | `/dunning/run` | Run dunning process |
| `GET` | `/dunning/pending` | List invoices in dunning |
| `GET` | `/dunning/{partner_id}/history` | Dunning history for customer |

### Aging & Reporting

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/aging/summary` | Customer aging summary by period |
| `GET` | `/aging/detail` | Detailed aging per customer |
| `GET` | `/aging/{partner_id}` | Aging for specific customer |
| `GET` | `/collections/pipeline` | Collections pipeline report |

---

## Odoo Technical Patterns to Follow

### Pattern 1: Invoice States Mirror Bills
Odoo's `account.invoice` mirrors the same state machine as bills: DRAFT → VALIDATED → POSTED → PAID. Validation computes taxes and final amounts. Posting creates the GL entry. Payment registration updates residual.

**Recommendation: RERP should use the same state machine. Each state transition is a separate POST endpoint for audit trail integrity.**

### Pattern 2: Payment Reconciliation is a Separate Step
Odoo separates payment registration from invoice posting. A payment can be registered against one or more invoices. Partial payments are fully supported. Multiple partial payments can cover one invoice.

**Recommendation: RERP must implement `register-payment` as a separate endpoint from `create-payment`. This enables partial and multi-invoice payments.**

### Pattern 3: Credit Limit Check on Sales Order
Odoo checks credit limits at sales order confirmation, not at invoice creation. If the order exceeds the customer's credit limit, it triggers a warning or blocks confirmation.

**Recommendation: RERP should integrate credit limit checking with the sales/order module. The credit check should consider open invoices, not just posted ones.**

### Pattern 4: Dunning is Periodic, Not Real-Time
Odoo's dunning is a batch process — you run it (daily/weekly) and it generates emails/reminders for overdue invoices. It's not real-time notification.

**Recommendation: RERP should implement dunning as a scheduled batch process. Dunning rules define the escalation path (email → letter → call → legal).**

---

## Competitive Positioning

### Where RERP Wins
- **API-native payment acceptance** — Online payments are processed via API, not embedded widgets. Easier integration, no vendor lock-in.
- **OpenAPI-defined credit limits** — Credit rules are defined in API specs, not GUI configuration.
- **Rust-level reconciliation speed** — Auto-matching thousands of payments to invoices in Rust is instantaneous.

### Where RERP Lags
- **No invoice entity defined** — Zero fields for customer invoicing.
- **No online payment integration** — No payment gateway integration (Stripe, PayPal).
- **No credit management** — No credit limits or holds.
- **No dunning/collections** — No automated reminders or escalation.

---

## Competitive Intelligence Deep Dive

### Oracle NetSuite: Comprehensive AR Suite
NetSuite's AR includes **automated invoicing** from orders, subscriptions, and time/billing. **Payment processing** supports credit cards, ACH, wire, and virtual cards. **Dunning management** automates email reminders with customizable templates. **Revenue recognition** handles ASC 606/IFRS 15 compliance. **Customer portal** lets customers view invoices, make payments, and download statements. **Bad debt management** with write-off approval workflows.

### SAP S/4HANA: Real-Time Collections
SAP's **Collections Management** uses Fiori apps with customer health scores, AI-powered payment prediction, and collection job management. **Automated dunning** runs by company code with configurable thresholds. **Payment proposal** optimizes collection timing. **Down payment processing** for advance payments. **Bank data integration** for real-time payment status.

### Odoo: Simple but Effective
Odoo's customer invoices auto-generate from sales orders. **Payment registration** links to payments with full reconciliation. **Online payment** integration with Stripe, PayPal, and Adyen. **Aging reports** by customer and period. **Customer portal** for self-service. **Recurring invoices** for subscriptions. **Credit limit** warnings at order confirmation. Simple enough for SMBs, powerful enough for mid-market.

### QuickBooks Online: Basic AR
QBO invoices are simple: create invoice, send email, record payment. **Payment acceptance** via integrated gateway. **Automated reminders** for overdue invoices. **Aging reports** in Basic plan, detailed in Advanced. **Recurring invoices** for subscriptions. Limited: no multi-level dunning, no payment portal, no credit management.

### Sage Intacct: Enterprise AR
Sage Intacct offers **automated dunning** with customizable email templates. **Collections management** with customer priority scoring. **Payment processing** with multiple methods. **Credit management** with automated holds. **Revenue recognition** for complex contracts. **Aging reports** by customer, product, salesperson, and custom dimensions. **Customer statements** via email or print.

### Xero: Lightweight AR
Xero's AR is simple: create invoice, send email, record payment. **Online payment** via Stripe/PayPal. **Automated reminders** for overdue invoices. **Aging reports** by overdue period. **Recurring invoices** for subscriptions. No dunning automation, no credit management, no payment portal.

### Zoho Books: Value-AR
Zoho Books offers **online payment forms** embedded in invoices. **Automated reminders** for overdue invoices. **Credit limit** management per customer. **Recurring invoices** for subscriptions. **Customer portal** for self-service. **Aging reports** with period breakdowns. Good value at $15-$125/mo with surprisingly comprehensive AR features.

---

## Implementation Roadmap

### Phase 1: Core Invoice Model (2-3 weeks) — P0
1. Define `CustomerInvoice` entity with full lifecycle states
2. Define `InvoiceLine` entity with product/expense linkage
3. Implement invoice → GL entry auto-posting
4. Implement invoice numbering and sequencing
5. Seed default revenue accounts and journals

### Phase 2: Payments & Reconciliation (2-3 weeks) — P0
1. Define `CustomerPayment` entity with reconciliation to invoices
2. Implement payment registration endpoint (single and multi-invoice)
3. Implement partial payment support
4. Implement residual calculation (`amount_residual = amount_total - amount_paid`)
5. Implement payment status tracking

### Phase 3: Credit Management & Aging (2 weeks) — P1
1. Define credit limit entity with available/used tracking
2. Implement credit check endpoint
3. Implement customer aging summary endpoint
4. Implement credit hold/release workflow
5. Implement overdue invoice tracking

### Phase 4: Dunning & Advanced Features (3 weeks) — P1
1. Define dunning rules with templates and escalation levels
2. Implement dunning batch runner
3. Implement automated email reminders
4. Add online payment gateway integration (Stripe/PayPal)
5. Implement customer portal for self-service payments

---

## Key Takeaway for Buyers

AR automation is the bridge between revenue and cash flow. A buyer should ask: *Can I invoice efficiently, accept payments online, and collect what's owed without chasing customers manually?* RERP's open API model means payment acceptance, dunning workflows, and credit management are all programmable. The gap with NetSuite/SAP is the full collections engine (dunning automation, payment prediction, customer health scores). But for organizations that want API-first control, RERP delivers the foundation with full customization potential.

**The immediate priority: define the CustomerInvoice entity with full state machine, payment reconciliation, and aging tracking. Everything in AR flows from there.**
