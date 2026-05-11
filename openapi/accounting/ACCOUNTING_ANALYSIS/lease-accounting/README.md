# Lease Accounting (ASC 842 / IFRS 16)

> **Component:** Lease registry, right-of-use asset calculation, lease liability, payment schedules, and modifications
> **Priority:** P3 — Required for organizations with significant leased assets (real estate, equipment, vehicles)
> **Odoo Reference:** Basic lease tracking via recurring bills; no native lease accounting standard support

---

## The Pitch

**Buyer Question:** *Can I track every lease, calculate the right-of-use asset and lease liability correctly, and automatically generate the required journal entries for ASC 842/IFRS 16 compliance?*\

Lease accounting standards (ASC 842 in the US, IFRS 16 internationally) require organizations to bring most leases onto the balance sheet — recognizing both a right-of-use asset and a lease liability. This eliminates the old "operating lease" off-balance-sheet treatment. This component handles the complete lease lifecycle: lease registry, initial measurement (PV of payments), subsequent measurement (amortization + interest), modifications, and termination.

---

## What This Component Does

Lease Accounting is the regulatory compliance layer over lease management. It handles:

1. **Lease Registry** — Complete inventory of all leases (real estate, equipment, vehicles)
2. **Initial Measurement** — Calculate PV of lease payments to establish ROU asset and lease liability
3. **Subsequent Measurement** — Monthly amortization of ROU asset and interest on lease liability
4. **Lease Modifications** — Handle changes in lease terms, rent escalations, term extensions
5. **Lease Terminations** — Record early termination or expiration with gain/loss calculation
6. **Disclosure Support** — Generate disclosure data for financial statement notes

---

## Entity Model

### Lease Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Lease reference (e.g., "HQ Office Lease 2025") |
| `lessor_id` | Foreign Key: Partner | Yes | Lessor (landlord/lessor company) |
| `type` | Enum: [REAL_ESTATE, EQUIPMENT, VEHICLE, OTHER] | Yes | Lease type |
| `classification` | Enum: [FINANCE, OPERATING] | Yes | ASC 842 classification |
| `company_id` | Foreign Key: Company | Yes | Lessee company |
| `currency_id` | Foreign Key: Currency | Yes | Lease currency |
| `lease_start_date` | Date | Yes | Commencement date |
| `lease_end_date` | Date | Yes | Term end date |
| `lease_term_months` | Integer | Computed | Total lease term in months |
| `remaining_term_months` | Integer | Computed | Remaining months |
| `payment_frequency` | Enum: [MONTHLY, QUARTERLY, ANNUALLY, OTHER] | Yes | Payment frequency |
| `base_rent_amount` | Decimal (15,2) | Yes | Base monthly/period rent |
| `escalation_clause` | Text | No | Rent escalation terms |
| `discount_rate` | Float | Yes | Incremental borrowing rate (%) |
| `discount_rate_source` | Enum: [ENTITY_SPECIFIC, RISK_FREE, LIBOR, SOFR] | No | Rate source |
| `initial_liability` | Decimal (15,2) | Computed | Initial PV of payments |
| `initial_rou_asset` | Decimal (15,2) | Computed | Initial ROU asset (liability + prepayments) |
| `liability_balance` | Decimal (15,2) | Computed | Current liability balance |
| `rou_asset_balance` | Decimal (15,2) | Computed | Current ROU asset balance |
| `accumulated_amortization` | Decimal (15,2) | Computed | Total amortization recorded |
| `state` | Enum: [ACTIVE, MODIFIED, TERMINATED, EXPIRED] | Yes | Lease state |
| `expense_account_id` | Foreign Key: Account | Yes | Rent expense account |
| `liability_account_id` | Foreign Key: Account | Yes | Lease liability account |
| `rou_asset_account_id` | Foreign Key: Account | Yes | ROU asset account |
| `amortization_method` | Enum: [STRAIGHT_LINE, DECLINING, CUSTOM] | Yes | ROU amortization method |
| `create_uid` | Foreign Key: User | Computed | Creator |
| `create_date` | DateTime | Computed | Created timestamp |

**Total fields: ~28.**

### Lease Payment Schedule Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `lease_id` | Foreign Key: Lease | Yes | Parent lease |
| `payment_date` | Date | Yes | Payment due date |
| `payment_amount` | Decimal (15,2) | Yes | Scheduled payment |
| `principal_portion` | Decimal (15,2) | Computed | Principal reduction |
| `interest_portion` | Decimal (15,2) | Computed | Interest expense |
| `paid` | Boolean | Yes | Payment made? |
| `paid_date` | Date | No | Actual payment date |
| `paid_amount` | Decimal (15,2) | No | Actual amount paid |
| `status` | Enum: [PENDING, PAID, OVERDUE, FORGIVEN] | Yes | Payment status |
| `invoice_id` | Foreign Key: Invoice | No | Linked invoice |

**Total fields: ~10.**

### Lease Journal Entry Entity

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `lease_id` | Foreign Key: Lease | Yes | Parent lease |
| `date` | Date | Yes | Entry date |
| `entry_type` | Enum: [INITIAL_RECOGNITION, MONTHLY_AMORTIZATION, INTEREST_ACCRUAL, PAYMENT, MODIFICATION, TERMINATION] | Yes | Entry type |
| `move_id` | Foreign Key: Move | No | Generated journal entry |
| `principal_amount` | Decimal (15,2) | No | Principal portion |
| `interest_amount` | Decimal (15,2) | No | Interest portion |
| `amortization_amount` | Decimal (15,2) | No | ROU amortization |
| `state` | Enum: [DRAFT, POSTED, CANCELLED] | Yes | Entry state |

**Total fields: ~9.**

---

## Required API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/leases` | List all leases |
| `GET` | `/leases/{id}` | Get lease detail |
| `POST` | `/leases` | Create lease |
| `PATCH` | `/leases/{id}` | Update lease |
| `POST` | `/leases/{id}/calculate-schedule` | Generate payment schedule |
| `GET` | `/leases/{id}/schedule` | View payment schedule |
| `POST` | `/leases/{id}/run-monthly` | Run monthly amortization + interest |
| `POST` | `/leases/{id}/modify` | Record lease modification |
| `POST` | `/leases/{id}/terminate` | Record lease termination |
| `GET` | `/leases/{id}/disclosures` | Generate disclosure data |

---

## Competitive Intelligence

**NetSuite:** Lease accounting module with ASC 842/IFRS 16 support. Automated lease liability calculation. ROU asset amortization. Lease modifications handling. Disclosure reporting.

**SAP S/4HANA:** Asset Accounting integrated with lease management. Lease liability tracking. ROU asset depreciation. Real-time lease accounting entries. Group reporting for multi-entity leases.

**Odoo:** No native ASC 842/IFRS 16 support. Manual journal entries for lease accounting. Third-party modules available.

**QuickBooks Online:** No native lease accounting. Manual journal entries required. Third-party apps (LeasePoint, LeaseWeb) add lease tracking.

**Sage Intacct:** Lease accounting with automated journal entries. ASC 842/IFRS 16 compliance. Lease modification handling. Disclosure reporting.

**Xero:** No native lease accounting. Manual entries required.

**Zoho Books:** No native lease accounting. Manual entries required.

---

## Implementation Roadmap

### Phase 1: Lease Registry & Initial Measurement (3 weeks) — P3
1. Define `Lease` entity with classification (finance vs operating)
2. Implement PV calculation for initial lease liability
3. Implement ROU asset calculation (liability + prepayments + initial costs)
4. Generate initial recognition journal entry

### Phase 2: Monthly Amortization & Interest (3 weeks) — P3
1. Define payment schedule entity with principal/interest breakdown
2. Implement monthly amortization of ROU asset
3. Implement interest accrual on lease liability
4. Generate monthly journal entries automatically

### Phase 3: Modifications & Disclosures (2 weeks) — P3
1. Implement lease modification handling (term change, rent change)
2. Implement lease termination with gain/loss calculation
3. Generate ASC 842/IFRS 16 disclosure data

---

## Key Takeaway for Buyers

Lease accounting compliance is mandatory for most organizations with leased assets. A buyer should ask: *Can my system track every lease, calculate the right-of-use asset and lease liability automatically, and generate compliant journal entries?* RERP's API-first model means lease data is fully programmable and audit-ready. The gap with NetSuite/SAP is the depth of ASC 842/IFRS 16 automation (lease modification handling, disclosure reporting). But for organizations that want full control over lease accounting with API access, RERP provides the foundation.

**The immediate priority: define Lease entity with initial measurement (PV calculation) and monthly amortization schedule.**
