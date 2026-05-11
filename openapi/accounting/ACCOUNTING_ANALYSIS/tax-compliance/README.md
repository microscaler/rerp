# Tax Compliance & Filing

> **Component:** Tax period management, jurisdiction rules, statutory returns, payment processing, and audit packs
> **Priority:** P2 — Multi-jurisdiction tax compliance is complex but essential for operating in multiple regions
> **Odoo Reference:** account.tax (Tax Engine, 4,000+ lines), account.fiscal.position (Fiscal Positions, 1,000+ lines)

---

## The Pitch

**Buyer Question:** *Can I calculate the correct tax on every transaction, file statutory returns on time, handle multi-jurisdiction rules, and produce audit-ready tax documentation?*\

Tax compliance is one of the most consequential areas of accounting — getting it wrong means penalties, interest, and potential legal exposure. This component handles tax calculation on every transaction, tax period management, statutory return filing, tax payment processing, and audit documentation. Multi-jurisdiction support is critical for organizations operating across states, countries, or tax zones.

---

## What This Component Does

Tax Compliance is the regulatory layer over all financial transactions. It handles:

1. **Tax Engine** — Calculate the correct tax on every invoice, payment, and journal entry
2. **Tax Periods** — Manage filing periods for VAT/GST, income tax, payroll tax, sales tax
3. **Statutory Returns** — Generate and file tax returns (VAT, GST, sales tax, corporate tax)
4. **Tax Payments** — Process tax payments to authorities with tracking and reconciliation
5. **Multi-Jurisdiction** — Handle different tax rules for different jurisdictions
6. **Fiscal Positions** — Map transactions to correct tax treatment based on location/type
7. **Audit Packs** — Assemble documentation for tax authority audits

---

## Entity Model

### Tax Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Tax name (e.g., "VAT 20%", "Sales Tax 8.25%") |
| `code` | String (16) | Yes | Tax code |
| `type_tax_use` | Enum: [NONE, SALE, PURCHASE, ALL] | Yes | Applicable to sales, purchases, or both |
| `amount` | Decimal (5,2) | Yes | Tax rate (%) |
| `amount_type` | Enum: [PERCENT, FIXED, ARRAY, DIVIDE] | Yes | Calculation type |
| `tax_group_id` | Foreign Key: Tax Group | Yes | Tax group (VAT, Sales Tax, etc.) |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `currency_id` | Foreign Key: Currency | No | Tax currency |
| `price_include` | Boolean | No | Is tax included in price? |
| `include_all_basis` | Boolean | No | Include in all tax bases? |
| `active` | Boolean | No | Soft delete |
| `description` | Text | No | Tax description |
| `jurisdiction_id` | Foreign Key: Jurisdiction | No | Tax jurisdiction |
| `report_group_id` | Foreign Key: Tax Group | No | Report group for returns |
| `refund_method` | Enum: [CARRYFORWARD, REFUND, CREDIT] | No | How to handle overpayments |
| `refunds_to_journal_id` | Foreign Key: Journal | No | Journal for refund entries |
| `tag_ids` | Many2Many: Tag | No | Classification tags |
| `create_uid` | Foreign Key: User | Computed | Creator |
| `create_date` | DateTime | Computed | Created timestamp |

**Total fields: ~20.**

### Tax Group Entity

Group related taxes (e.g., multiple VAT rates):

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Group name (e.g., "VAT", "Sales Tax") |
| `aggregate` | String (64) | Yes | Aggregation field (sum, average, etc.) |
| `description` | Text | No | Group description |
| `country_group_id` | Foreign Key: Country Group | No | Applicable countries |
| `is_circular_detection` | Boolean | No | Enable circular dependency detection |

**Total fields: ~6.**

### Tax Period Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (64) | Yes | Period name (e.g., "Q1 2025 VAT") |
| `code` | String (16) | Yes | Period code |
| `type` | Enum: [VAT, GST, SALES_TAX, CORPORATE_TAX, PAYROLL_TAX] | Yes | Tax type |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `date_from` | Date | Yes | Period start |
| `date_to` | Date | Yes | Period end |
| `state` | Enum: [DRAFT, DRAFT_FILED, FILED, PAID, CLOSED] | Yes | Period state |
| `tax_base_amount` | Decimal (15,2) | Computed | Total taxable amount |
| `tax_amount` | Decimal (15,2) | Computed | Total tax due |
| `amount_paid` | Decimal (15,2) | Computed | Amount paid to authority |
| `amount_due` | Decimal (15,2) | Computed | Remaining due |
| `filing_date` | DateTime | No | Actual filing date |
| `payment_date` | DateTime | No | Actual payment date |

**Total fields: ~14.**

### Tax Return Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `tax_period_id` | Foreign Key: Tax Period | Yes | Parent period |
| `name` | String (128) | Yes | Return reference |
| `type` | Enum: [VAT, GST, SALES_TAX, CORPORATE_TAX, PAYROLL_TAX] | Yes | Return type |
| `jurisdiction_id` | Foreign Key: Jurisdiction | Yes | Tax authority |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `tax_base` | Decimal (15,2) | Yes | Total taxable sales/purchases |
| `tax_deductible` | Decimal (15,2) | Yes | Tax deducted (input tax) |
| `tax_collected` | Decimal (15,2) | Yes | Tax collected (output tax) |
| `tax_due` | Decimal (15,2) | Computed | Net tax due (collected - deductible) |
| `amount_paid` | Decimal (15,2) | No | Amount paid |
| `filing_status` | Enum: [DRAFT, SUBMITTED, ACCEPTED, REJECTED] | Yes | Filing status |
| `filing_method` | Enum: [MANUAL, ELECTRONIC, API] | Yes | How filed |
| `reference` | String (255) | No | Authority reference number |
| `attachment` | Binary | No | Filed return PDF |

**Total fields: ~15.**

### Tax Payment Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `tax_period_id` | Foreign Key: Tax Period | Yes | Related period |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `tax_authority_id` | Foreign Key: Partner | Yes | Tax authority |
| `payment_date` | Date | Yes | Payment date |
| `amount` | Decimal (15,2) | Yes | Payment amount |
| `currency_id` | Foreign Key: Currency | Yes | Payment currency |
| `payment_method` | Enum: [ACH, WIRE, CHECK, ONLINE] | Yes | Payment method |
| `reference` | String (255) | No | Payment reference |
| `move_id` | Foreign Key: Move | No | GL entry |
| `state` | Enum: [DRAFT, CONFIRMED, PAID] | Yes | Payment state |

**Total fields: ~11.**

---

## Required API Endpoints

### Tax Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/taxes` | List all taxes |
| `GET` | `/taxes/{id}` | Get tax detail |
| `POST` | `/taxes` | Create tax |
| `PATCH` | `/taxes/{id}` | Update tax |

### Tax Calculation

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/taxes/calculate` | Calculate tax on amounts |
| `POST` | `/taxes/calculate-line` | Calculate tax for invoice line |
| `GET` | `/taxes/breakdown/{invoice_id}` | Get tax breakdown for invoice |
| `GET` | `/taxes/jurisdiction/{country}/{zip}` | Get applicable taxes by location |

### Tax Periods & Returns

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/tax-periods` | List tax periods |
| `POST` | `/tax-periods` | Create tax period |
| `PATCH` | `/tax-periods/{id}/close` | Close tax period |
| `GET` | `/tax-periods/{id}/summary` | Period tax summary |
| `POST` | `/tax-periods/{id}/generate-return` | Generate tax return |
| `POST` | `/tax-periods/{id}/file` | File tax return (electronic) |

### Tax Payments

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/tax-payments` | List tax payments |
| `POST` | `/tax-payments` | Create tax payment |
| `GET` | `/taxes/outstanding` | Outstanding tax due |

### Audit Packs

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/tax/audit-pack/{period_id}` | Generate audit documentation pack |
| `GET` | `/tax/audit-pack/{id}` | Get audit pack |

---

## Competitive Intelligence

**NetSuite:** Automated tax calculation with Avalara integration. Multi-jurisdiction filing. E-invoicing for tax authorities. Tax reporting by jurisdiction. Corporate tax reporting with BEPS compliance.

**SAP S/4HANA:** Real-time tax calculation with 500+ tax jurisdictions. Automatic tax determination from master data. Fiori apps for tax reporting. Integration with tax authority portals.

**Odoo:** Tax engine with multiple rates, groups, and fiscal positions. VAT report auto-generation. Tax period management. AvaTax integration for advanced compliance.

**QuickBooks Online:** Automatic tax calculation based on customer location. Sales tax tracking and reporting. 1099 forms. Limited multi-jurisdiction support.

**Sage Intacct:** Multi-jurisdiction tax handling. Automated sales tax calculation. Tax exemption certificates. Integration with Avalara for advanced compliance.

**Xero:** Automatic tax rates by country. Sales tax reporting. VAT return generation for supported countries. Limited multi-jurisdiction.

**Zoho Books:** Tax rates by location. VAT/GST support. Tax exemption handling. Tax reports for various jurisdictions. Good international tax support at low cost.

---

## Competitive Positioning

### Where RERP Wins
- **OpenAPI-defined tax rules** — Tax logic is machine-readable, enabling automatic compliance across jurisdictions.
- **Rust-level tax calculation** — Tax on millions of transactions is instant.
- **Self-hosted, no Avalara subscription** — No per-transaction tax calculation fees.

### Where RERP Lags
- **No tax entities defined** — Zero fields for tax calculation.
- **No tax engine** — No rate determination or calculation logic.
- **No filing integration** — No connection to tax authority portals.

---

## Implementation Roadmap

### Phase 1: Tax Engine (3 weeks) — P2
1. Define `Tax` entity with rate types and jurisdiction
2. Define `TaxGroup` entity for multi-rate taxes
3. Implement tax calculation on transaction lines
4. Implement tax breakdown on invoices

### Phase 2: Tax Periods & Returns (3 weeks) — P2
1. Define `TaxPeriod` entity with filing states
2. Implement tax return generation (aggregate periods)
3. Implement tax payment processing
4. Add tax reporting by jurisdiction

---

## Key Takeaway for Buyers

Tax compliance is non-negotiable but rarely exciting. A buyer should ask: *Will my system calculate the right tax, file on time, and produce audit-ready documentation?* RERP's API-first model means tax rules are fully programmable and audit trails are complete. The gap with NetSuite/SAP is the breadth of jurisdictional coverage and automated filing integrations. But for organizations that want full control over tax logic with complete API access, RERP provides the foundation.

**The immediate priority: define the Tax entity with calculation engine. Tax on every transaction is the foundation; everything else (periods, returns, payments) flows from there.**
