# Vendor Invoice & Payment Management

> **Component:** AP lifecycle — vendor master, invoice capture (manual/OCR/email), approval workflows, three-way match, payment execution, and aging
> **Priority:** P0 — Cash outflow management is as critical as revenue collection
> **Odoo Reference:** account.bill (3,000+ lines), res.partner (vendor-specific), account.payment (1,500 lines)

---

## The Pitch

**Buyer Question:** *Can I capture every vendor bill automatically, route it through the right approval chain, match it to orders and receipts, and pay vendors on time without late fees — all while maintaining complete audit visibility?*\

If you're manually tracking vendor invoices in spreadsheets or email inboxes, you're bleeding money through late fees, missed discounts, and duplicate payments. AP automation is one of the highest-ROI use cases in accounting. This component handles the full AP lifecycle from vendor onboarding to payment execution, including OCR document ingestion, three-way matching (PO ↔ receipt ↔ invoice), approval workflows, and scheduled payments.

---

## What This Component Does

Vendor Payables is the controlled outflow of cash. It handles:

1. **Vendor Master** — Complete vendor directory with payment terms, banking details, tax info, and credit limits
2. **Invoice Capture** — Manual entry, email ingestion, OCR from PDFs/scans, EDI (PEPPOL/UBL)
3. **Approval Workflows** — Route invoices for approval based on amount, department, or vendor
4. **Three-Way Match** — Validate invoice against purchase order and received goods before paying
5. **Payment Execution** — Schedule payments, batch process, SEPA/ACH/wire transfer generation
6. **Credit Management** — Vendor credit notes, refunds, and payment term tracking
7. **Aging Analysis** — Track payables aging (current, 30, 60, 90+ days overdue)

---

## Entity Model

### Vendor Bill Entity

The core invoice record from a vendor:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Bill reference number (auto-generated, e.g., "BILL/2025/0001") |
| `vendor_bill_number` | String (128) | Yes | Original vendor invoice number |
| `partner_id` | Foreign Key: Partner | Yes | Vendor (supplier) |
| `journal_id` | Foreign Key: Journal | Yes | Purchase journal |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `currency_id` | Foreign Key: Currency | Yes | Invoice currency |
| `invoice_date` | Date | Yes | Invoice date from vendor |
| `invoice_date_due` | Date | Yes | Payment due date |
| `invoice_date_due_days` | Integer | Computed | Days until due |
| `amount_untaxed` | Decimal (15,2) | Yes | Subtotal before tax |
| `amount_tax` | Decimal (15,2) | Yes | Total tax amount |
| `amount_total` | Decimal (15,2) | Yes | Grand total (untaxed + tax) |
| `amount_paid` | Decimal (15,2) | Computed | Total amount already paid |
| `amount_residual` | Decimal (15,2) | Computed | Remaining balance (total - paid) |
| `amount_tax_included` | Boolean | No | Are taxes included in line prices? |
| `state` | Enum: [DRAFT, VALIDATED, POSTED, PAID, CANCELLED, REVERSED] | Yes | Bill lifecycle |
| `move_id` | Foreign Key: Move | Computed | Associated journal entry |
| `move_type` | Enum: [INVOICE, DEBIT_NOTE, CREDIT_NOTE, RETURN] | Yes | Bill type |
| `payment_reference` | String (255) | No | Payment reference for vendor |
| `ref` | String (255) | No | Internal reference |
| `narration` | Text | No | Bill description |
| `account_payment_term_id` | Foreign Key: Payment Term | No | Payment terms (Net 30, etc.) |
| `purchase_order_id` | Foreign Key: PO | No | Linked purchase order (for 3-way match) |
| `payment_status` | Enum: [UNPAID, PARTIALLY_PAID, IN_PAYMENT, FULLY_PAID] | Computed | Derived from amount_paid |
| `invoice_line_ids` | One2Many: Bill Line | Yes | Line items |
| `tax_totals` | JSON | No | Tax breakdown per line |
| `document_url` | String (512) | No | URL to scanned document |
| `document_pages` | Integer | No | Number of pages (from OCR) |
| `extracted_vendor_name` | String (255) | No | Extracted from OCR |
| `extracted_invoice_number` | String (128) | No | Extracted from OCR |
| `extracted_invoice_date` | Date | No | Extracted from OCR |
| `extracted_amount_total` | Decimal (15,2) | No | Extracted from OCR |
| `extracted_confidence` | Float (0-1) | No | OCR confidence score |
| `needs_review` | Boolean | Computed | True if OCR data needs manual review |
| `create_uid` | Foreign Key: User | Computed | Who created |
| `create_date` | DateTime | Computed | When created |
| `write_uid` | Foreign Key: User | Computed | Last modifier |
| `write_date` | DateTime | Computed | Last modified |

**Total fields: ~40.**

### Vendor Bill Line Entity

Individual line items on a vendor bill:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `move_id` | Foreign Key: Move | Yes | Parent bill entry |
| `sequence` | Integer | Yes | Display order |
| `product_id` | Foreign Key: Product | No | Product/service |
| `name` | String (255) | No | Line description |
| `quantity` | Float | Yes | Quantity |
| `price_unit` | Decimal (15,2) | Yes | Unit price |
| `price_subtotal` | Decimal (15,2) | Computed | quantity × price_unit |
| `price_total` | Decimal (15,2) | Computed | subtotal + taxes |
| `tax_ids` | Many2Many: Tax | No | Taxes applied |
| `account_id` | Foreign Key: Account | Yes | Expense account |
| `analytic_distribution` | JSON | No | Cost center breakdown |
| `vendor_id` | Foreign Key: Partner | No | Vendor (from parent) |
| `discount` | Decimal (15,2) | No | Discount amount |
| `discount_percent` | Decimal (5,2) | No | Discount percentage |
| `product_uom_id` | Foreign Key: UOM | No | Unit of measure |
| `move_line_id` | Foreign Key: Move Line | No | Linked GL line |

**Total fields: ~17.**

### Vendor Payment Entity

Payment made to a vendor:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `payment_date` | Date | Yes | Payment date |
| `amount` | Decimal (15,2) | Yes | Payment amount |
| `currency_id` | Foreign Key: Currency | Yes | Payment currency |
| `partner_id` | Foreign Key: Partner | Yes | Vendor recipient |
| `journal_id` | Foreign Key: Journal | Yes | Payment journal (Bank/Cash) |
| `company_id` | Foreign Key: Company | Yes | Payer company |
| `payment_method_id` | Foreign Key: Method | Yes | SEPA/ACH/Wire/Check |
| `payment_reference` | String (255) | Yes | Payment reference |
| `bank_transaction_id` | Foreign Key: Transaction | No | Linked bank transaction |
| `move_id` | Foreign Key: Move | Computed | Associated GL entry |
| `state` | Enum: [DRAFT, CONFIRMED, SENT, PAID, CANCELLED] | Yes | Payment lifecycle |
| `invoice_ids` | Many2Many: Bill | Yes | Bills being paid |
| `payment_type` | Enum: [OUTBOUND, INBOUND, TRANSFER] | Yes | Payment direction |
| `payment_mode` | Enum: [MANUAL, ELECTRONIC, CHECK, CASH] | Yes | How payment is made |
| `communication` | String (255) | No | Payment communication to vendor |
| `bank_partner_id` | Foreign Key: Partner | Yes | Vendor bank account |
| `create_uid` | Foreign Key: User | Computed | Who created |
| `create_date` | DateTime | Computed | When created |

**Total fields: ~20.**

### Payment Term Entity

Define when vendors get paid:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Term name (e.g., "Net 30", "Due on Receipt") |
| `payment_action_id` | Foreign Key: Action | Yes | Action type (immediate, scheduled, end of month) |
| `sequence` | Integer | Yes | Display order |
| `create_uid` | Foreign Key: User | Computed | Creator |
| `create_date` | DateTime | Computed | Created timestamp |

**Total fields: ~5.** Simple but essential.

### Vendor Credit Note Entity

Credit received from a vendor (return/reversal):

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Credit note reference |
| `vendor_bill_number` | String (128) | Yes | Original vendor number |
| `partner_id` | Foreign Key: Partner | Yes | Vendor |
| `journal_id` | Foreign Key: Journal | Yes | Purchase journal |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `currency_id` | Foreign Key: Currency | Yes | Credit currency |
| `credit_date` | Date | Yes | Credit date |
| `amount_total` | Decimal (15,2) | Yes | Credit amount |
| `state` | Enum: [DRAFT, VALIDATED, POSTED, CANCELLED] | Yes | State |
| `move_id` | Foreign Key: Move | Computed | GL entry |
| `original_bill_id` | Foreign Key: Bill | No | Original bill being credited |
| `invoice_line_ids` | One2Many: Credit Line | Yes | Credit lines |
| `narration` | Text | No | Reason for credit |

**Total fields: ~13.**

---

## Entity Relationships

```
account.bill (vendor bills)
  ├── res.partner (partner_id)         ← Vendor
  ├── account.journal (journal_id)     ← Purchase journal
  ├── account.move (move_id)           ← GL entry
  ├── purchase.order (purchase_order_id) ← PO for 3-way match
  ├── account.payment.term (term)      ← Payment terms
  └── account.bill.line (line_ids)     ← Line items

account.bill.line
  ├── product.product (product_id)    ← Product/service
  ├── account.account (account_id)     ← Expense account
  └── account.move.line (move_line_id) ← GL line

account.payment (vendor payments)
  ├── res.partner (partner_id)         ← Vendor
  ├── account.journal (journal_id)     ← Payment journal (bank)
  ├── account.move (move_id)           ← GL entry
  └── account.bill (invoice_ids)       ← Bills being paid (reconciliation)
```

---

## Required API Endpoints

### Vendor Bill CRUD

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/bills` | List bills with filters (state, vendor, date range) |
| `GET` | `/bills/{id}` | Get bill detail with lines |
| `POST` | `/bills` | Create draft bill |
| `PATCH` | `/bills/{id}` | Update draft bill |
| `POST` | `/bills/{id}/validate` | Validate bill (transition to VALIDATED) |
| `POST` | `/bills/{id}/post` | Post bill (create GL entry, transition to POSTED) |
| `DELETE` | `/bills/{id}` | Cancel draft bill |
| `POST` | `/bills/import` | Bulk import from CSV |
| `POST` | `/bills/ocr-process` | Process OCR document |
| `POST` | `/bills/detect-duplicates` | Detect duplicate bills |

### Three-Way Match

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/bills/{id}/match` | Validate 3-way match (PO ↔ receipt ↔ invoice) |
| `GET` | `/bills/{id}/match-status` | Check match status |
| `GET` | `/bills/mismatches` | List bills with match failures |

### Vendor Payments

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/payments` | List vendor payments |
| `POST` | `/payments` | Create payment |
| `POST` | `/payments/{id}/confirm` | Confirm payment |
| `POST` | `/payments/batch` | Create batch payments |
| `GET` | `/bills/unpaid` | List unpaid bills |
| `POST` | `/bills/{id}/pay` | Create payment for specific bill |
| `POST` | `/payments/{id}/register-batch` | Register payment for multiple bills |

### Aging & Reporting

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/aging/summary` | Vendor aging summary by period |
| `GET` | `/aging/detail` | Detailed aging per vendor |
| `GET` | `/aging/{partner_id}` | Aging for specific vendor |

### Credit Notes

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/credit-notes` | List vendor credit notes |
| `POST` | `/credit-notes` | Create credit note |
| `POST` | `/credit-notes/{id}/post` | Post credit note |

### Payment Terms

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/payment-terms` | List payment terms |
| `POST` | `/payment-terms` | Create payment term |
| `PATCH` | `/payment-terms/{id}` | Update payment term |

---

## Odoo Technical Patterns to Follow

### Pattern 1: Bill States Drive Behavior
Odoo's `account.bill` moves through states: DRAFT → VALIDATED → POSTED → PAID. Each state transition triggers specific behavior — validation computes taxes, posting creates GL entries, payment reconciliation updates residual amounts.

**Recommendation: RERP should implement the same state machine. Each transition is a POST endpoint, not a PATCH. This enforces proper audit trails.**

### Pattern 2: Payment Reconciliation Updates Residual
When a payment is registered against a bill, Odoo links them and computes `amount_residual = amount_total - amount_paid`. Multiple partial payments are tracked. Once `amount_residual = 0`, `payment_status` transitions to `FULLY_PAID`.

**Recommendation: RERP must compute residual on read. Never update `amount_residual` directly — always derive from sum of linked payments.**

### Pattern 3: Three-Way Match is Configurable
Odoo allows configuration of matching criteria: PO-only, PO+Receipt, or PO+Receipt+Invoice. When match fails, the bill can still be posted but is flagged.

**Recommendation: RERP should make 3-way match configurable per vendor or per bill. Failures should produce warnings, not hard blocks.**

### Pattern 4: Duplicate Detection via OCR Reference
Odoo flags potential duplicates when `vendor_bill_number` matches a prior bill or when OCR-extracted data (amount, date, vendor) overlaps significantly.

**Recommendation: RERP should implement duplicate detection on both `vendor_bill_number` and OCR-extracted fields. Flag duplicates for review before posting.**

---

## Competitive Positioning

### Where RERP Wins
- **API-native invoice capture** — Invoices arrive via API or webhook, not email parsing. Structured data from day one.
- **OpenAPI-defined approval workflows** — Approval chains are defined in API specs, not GUI configuration.
- **Rust-level matching speed** — Three-way matching across thousands of POs and receipts is instantaneous.

### Where RERP Lags
- **No bill entity defined** — Zero fields for vendor invoice management.
- **No 3-way match** — No purchase order linkage or receipt matching.
- **No payment execution** — Payments are a separate concept with no reconciliation to bills.
- **No OCR pipeline** — Document ingestion and extraction is a separate service with no integration to bills.

---

## Competitive Intelligence Deep Dive

### Oracle NetSuite: Automated AP Workflow
NetSuite's AP module supports **automated invoice capture** from email, scanning, and EDI. **Three-way match** validates PO, receipt, and invoice. **Approval workflows** route based on amount thresholds. **Electronic payments** support ACH, wire, check, and virtual cards. **Predictive payments** suggest optimal payment timing to maximize discounts while preserving cash. **Bill splits** allow partial payments across cost centers.

### SAP S/4HANA: Intelligent Invoice Processing
SAP's **Intelligent Invoice Processing** uses AI/ML for automated posting, multi-channel capture (email, portal, fax), and 3-way matching. **Payment run** processes millions of vendor payments in minutes with optimized payment methods. **House bank management** handles multiple bank accounts, payment methods per country, and payment proposals. **Vendor master** integrates with procurement and logistics.

### Odoo: Simple but Effective
Odoo's vendor bills integrate seamlessly with purchase orders and inventory. **Auto-validation** can post bills that match POs. **Payment registration** links bills to payments with full reconciliation. **Vendor bills via email** — forward PDF to a dedicated address for auto-ingestion. **Batch payments** process all due bills in one click. Community edition handles most AP needs; Enterprise adds bank sync and smart matching.

### QuickBooks Online: Streamlined AP
QBO's bill feature is deliberately simple — enter bill, assign to vendor, set due date, mark as paid. **Vendor center** shows all bills, payments, and credits. **Bill pay** (add-on) automates check printing and bill pay. **Approve & pay** lets managers approve bills in the queue. Limited customization: no multi-level approval, no 3-way matching, no vendor portals.

### Sage Intacct: Enterprise AP
Sage Intacct supports **multi-level approval workflows** with escalation rules. **Automated 3-way match** with exception handling. **Electronic payment** with ACH, wire, check, and international payments. **Vendor management** with portal for self-service invoice submission. **AP analytics** dashboard shows spend by vendor, category, and cost center. **Check printing** with ACH conversion.

### Xero: Lightweight AP
Xero's AP is simple: enter bill, link to contact, set due date, pay. **Bank feed matching** auto-reconciles payments. **Multi-currency** with gain/loss tracking. **Recurring bills** automate regular payments. No 3-way matching, no multi-level approval, no vendor portal. Good for simple AP, insufficient for complex procurement.

### Zoho Books: Value-AP
Zoho Books offers **vendor portal** for self-service invoice submission. **Approval workflows** with configurable approval chains. **Recurring bills** for regular payments. **Payment reminders** for overdue vendor payments. **Bank feeds** with auto-matching. Good value at $15-$125/mo but limited advanced features.

---

## Implementation Roadmap

### Phase 1: Core Bill & Payment Model (2-3 weeks) — P0
1. Define `VendorBill` entity with full lifecycle states
2. Define `VendorBillLine` entity with product/expense account linkage
3. Define `VendorPayment` entity with reconciliation to bills
4. Define `PaymentTerm` entity (Net 30, Net 60, etc.)
5. Implement bill → GL entry auto-posting
6. Implement payment → bill reconciliation with residual calculation

### Phase 2: Invoice Capture & Matching (2-3 weeks) — P0
1. Integrate with `documents-extraction` service for OCR
2. Implement bill creation from OCR-extracted data
3. Implement duplicate detection on vendor bill number and OCR fields
4. Define `PurchaseOrder` linkage (3-way match fields on bill)
5. Implement match status tracking

### Phase 3: Payments & Aging (2 weeks) — P1
1. Implement batch payment creation
2. Implement payment state machine (DRAFT → CONFIRMED → PAID)
3. Implement vendor aging summary endpoint
4. Implement overdue payment alerts
5. Implement credit note processing

### Phase 4: Approval & Advanced Features (3 weeks) — P1
1. Implement approval workflow engine for bills by amount/vendor
2. Add vendor portal integration (invoice submission via API)
3. Implement payment scheduling (optimize discount capture vs cash)
4. Add vendor credit management
5. Implement payment method configuration (SEPA/ACH/wire)

---

## Key Takeaway for Buyers

AP automation is one of the clearest ROI stories in accounting software. A buyer should ask: *Can I capture invoices automatically, approve them efficiently, and pay vendors on time without manual data entry?* RERP's open API model means invoice capture, approval routing, and payment execution are all programmable — no GUI limitations, no vendor lock-in. The gap with NetSuite/SAP is the full workflow engine (approvals, 3-way match, payment scheduling). But for organizations that want full control and API access, RERP delivers the foundation with the flexibility to customize.

**The immediate priority: define the VendorBill entity with full state machine, GL auto-posting, and payment reconciliation. Everything in AP flows from there.**
