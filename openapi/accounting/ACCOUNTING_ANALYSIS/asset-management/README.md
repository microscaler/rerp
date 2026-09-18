# Asset Management & Depreciation

> **Component:** Fixed asset lifecycle — acquisition, depreciation scheduling, revaluation, transfer, impairment, and disposal
> **Priority:** P1 — Depreciation is legally required for accurate financial statements
> **Odoo Reference:** account.asset (2,000+ lines), account.asset.category (500+ lines)

---

## The Pitch

**Buyer Question:** *Can I track every fixed asset from purchase through depreciation to disposal, with automatic journal entries and compliance with accounting standards?*\

Fixed asset management ensures your balance sheet accurately reflects the value of equipment, buildings, vehicles, and other long-lived assets. This component handles the complete asset lifecycle: recording acquisitions, calculating depreciation automatically, handling transfers between locations, tracking revaluations, recording impairments, and managing disposals or sales.

---

## What This Component Does

Asset Management is the bridge between procurement and finance. It handles:

1. **Asset Register** — Complete inventory of all fixed assets with descriptions, locations, and custodians
2. **Depreciation Schedules** — Automatic calculation and posting of depreciation (straight-line, declining balance, units of production)
3. **Asset Categories** — Group assets by type with default depreciation rules
4. **Revaluations** — Update asset values based on appraisals or market conditions
5. **Transfers** — Move assets between locations, departments, or custodians
6. **Impairments** — Record losses in value when asset is impaired
7. **Disposals** — Sell, scrap, or retire assets with gain/loss calculation

---

## Entity Model

### Asset Category Entity

Defines depreciation rules for groups of assets:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Category name (e.g., "Machinery", "Vehicles", "Furniture") |
| `asset_depreciation_account_id` | Foreign Key: Account | Yes | Expense account for depreciation |
| `asset_income_depreciation_account_id` | Foreign Key: Account | No | Income account (if reversal) |
| `asset_counterpart_account_id` | Foreign Key: Account | Yes | Credit account on acquisition |
| `asset_income_account_id` | Foreign Key: Account | No | Gain on disposal account |
| `asset_sale_value_account_id` | Foreign Key: Account | No | Loss on disposal account |
| `depreciation_board` | Enum: [LIST, GRID] | No | Depreciation board display |
| `depreciation_type` | Enum: [GROUP_ENTRY, INDIVIDUAL_ENTRY] | No | Depreciation posting |
| `method` | Enum: [LINEAR, DECREASING, DECLINING, UNITS_OF_PRODUCTION] | Yes | Depreciation method |
| `method_percentage` | Float | Yes | Depreciation % (for declining balance) |
| `method_number` | Integer | No | Number of periods (for linear) |
| `method_period` | Enum: [YEAR, MONTH] | No | Depreciation period |
| `method_prefix` | String (16) | No | Prefix for depreciation entry |
| `method_suffix` | String (16) | No | Suffix for depreciation entry |
| `recoverable` | Boolean | No | Is depreciation tax recoverable? |

**Total fields: ~18.**

### Asset Entity

Individual fixed asset record:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `name` | String (128) | Yes | Asset name |
| `code` | String (64) | Yes | Asset reference code |
| `category_id` | Foreign Key: Category | Yes | Depreciation category |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `partner_id` | Foreign Key: Partner | No | Vendor (original supplier) |
| `acquisition_date` | Date | Yes | Date acquired |
| `acquisition_value` | Decimal (15,2) | Yes | Original cost |
| `currency_id` | Foreign Key: Currency | Yes | Asset currency |
| `salvage_value` | Decimal (15,2) | No | Estimated residual value |
| `depreciation_limit` | Decimal (15,2) | Computed | Acquisition - salvage |
| `value` | Decimal (15,2) | Computed | Current book value |
| `residual` | Decimal (15,2) | Computed | Residual value remaining |
| `state` | Enum: [NOT_CREATED, CREATED, NOT_DEPRECIATED, DEPRECIATING, CLOSE, CANCELLED] | Yes | Asset lifecycle |
| `parent_id` | Foreign Key: Asset | No | Parent asset (for group assets) |
| `group_id` | Foreign Key: Asset | No | Group parent |
| `location_id` | Foreign Key: Location | No | Asset location |
| `responsible_id` | Foreign Key: User | No | Asset custodian |
| `notes` | Text | No | Asset description/notes |
| `image` | Binary | No | Asset photo |
| `lifetime` | Integer | No | Expected useful life (months) |
| `depreciation_type` | Enum: [HOUR, MONTH, YEAR] | No | Depreciation period unit |
| `depreciation_days` | Integer | No | Days per year for depreciation |
| `create_uid` | Foreign Key: User | Computed | Creator |
| `create_date` | DateTime | Computed | Created timestamp |

**Total fields: ~26.**

### Asset Depreciation Line Entity

Individual depreciation entries:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `asset_id` | Foreign Key: Asset | Yes | Parent asset |
| `name` | String (255) | Yes | Depreciation entry name |
| `date` | Date | Yes | Depreciation date |
| `period_id` | Foreign Key: Period | Yes | Fiscal period |
| `amount` | Decimal (15,2) | Yes | Depreciation amount |
| `depreciated_value` | Decimal (15,2) | Computed | Accumulated depreciation |
| `remaining_value` | Decimal (15,2) | Computed | Book value after depreciation |
| `move_id` | Foreign Key: Move | No | Related journal entry |
| `asset_value` | Decimal (15,2) | Computed | Asset value at date |
| `asset_remaining_value` | Decimal (15,2) | Computed | Remaining value at date |
| `is_posted` | Boolean | Yes | Has GL entry been created? |
| `create_uid` | Foreign Key: User | Computed | Creator |
| `create_date` | DateTime | Computed | Created timestamp |

**Total fields: ~14.**

### Asset Revaluation Entity

Track value changes:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `asset_id` | Foreign Key: Asset | Yes | Asset being revalued |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `revaluation_date` | Date | Yes | Revaluation date |
| `previous_value` | Decimal (15,2) | Computed | Value before revaluation |
| `new_value` | Decimal (15,2) | Yes | New appraised value |
| `difference` | Decimal (15,2) | Computed | Difference (gain/loss) |
| `reserve_account_id` | Foreign Key: Account | Yes | Revaluation reserve account |
| `profit_loss_account_id` | Foreign Key: Account | Yes | P&L account |
| `move_id` | Foreign Key: Move | No | Journal entry |
| `state` | Enum: [DRAFT, POSTED, CANCELLED] | Yes | State |
| `narration` | Text | No | Reason for revaluation |

**Total fields: ~11.**

### Asset Disposal Entity

Record asset disposal:

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `id` | UUID | Yes | Primary key |
| `asset_id` | Foreign Key: Asset | Yes | Asset being disposed |
| `company_id` | Foreign Key: Company | Yes | Owner company |
| `disposal_date` | Date | Yes | Disposal date |
| `disposal_value` | Decimal (15,2) | Yes | Sale/scrap value |
| `currency_id` | Foreign Key: Currency | Yes | Disposal currency |
| `gain_loss` | Decimal (15,2) | Computed | Disposal value - book value |
| `gain_account_id` | Foreign Key: Account | No | Gain on disposal account |
| `loss_account_id` | Foreign Key: Account | No | Loss on disposal account |
| `disposal_type` | Enum: [SALE, SCRAPP, TRANSFER, DONATION] | Yes | Disposal method |
| `move_id` | Foreign Key: Move | No | Journal entry |
| `state` | Enum: [DRAFT, POSTED, CANCELLED] | Yes | State |
| `narration` | Text | No | Reason for disposal |

**Total fields: ~13.**

---

## Entity Relationships

```
account.asset.category
  ├── account.asset (category_id)              ← Assets belong to categories
  └── account.asset.depreciation.line (via category rules)

account.asset (individual assets)
  ├── account.asset.category (category_id)     ← Depreciation rules
  ├── account.asset.depreciation.line (lines)  ← Depreciation schedule
  ├── account.asset.revaluation (revaluations) ← Value changes
  └── account.asset.disposal (disposals)       ← End-of-life

account.asset.depreciation.line
  └── account.move (move_id)                   ← GL entry
```

---

## Required API Endpoints

### Asset CRUD

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/assets` | List all assets |
| `GET` | `/assets/{id}` | Get asset detail |
| `POST` | `/assets` | Create asset |
| `PATCH` | `/assets/{id}` | Update asset |
| `GET` | `/assets/category/{category_id}` | List assets by category |

### Depreciation

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/assets/{id}/compute` | Calculate depreciation schedule |
| `POST` | `/assets/{id}/depreciate` | Run depreciation for period |
| `POST` | `/assets/depreciate-all` | Run depreciation for all assets |
| `GET` | `/assets/{id}/depreciation-schedule` | View full depreciation schedule |
| `GET` | `/assets/{id}/depreciation-history` | Historical depreciation entries |

### Revaluation & Disposal

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/assets/{id}/revalue` | Revalue asset |
| `POST` | `/assets/{id}/dispose` | Dispose of asset |
| `GET` | `/assets/{id}/disposals` | List disposals |

### Reports

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/reports/assets/register` | Fixed asset register report |
| `GET` | `/reports/assets/depreciation-summary` | Depreciation summary by category |
| `GET` | `/reports/assets/disposal-summary` | Disposal summary |

---

## Competitive Positioning

### Where RERP Wins
- **OpenAPI-defined asset lifecycle** — Asset rules are machine-readable, enabling automatic compliance.
- **Rust-level depreciation calculation** — Computing depreciation for 10,000 assets across 10 years is instant.
- **Self-hosted, no per-asset fees** — Unlimited assets with no licensing restrictions.

### Where RERP Lags
- **No asset entities defined** — Zero fields for fixed asset management.
- **No depreciation engine** — No calculation or scheduling logic.
- **No revaluation or disposal tracking** — Complete lifecycle missing.

---

## Competitive Intelligence Deep Dive

### Oracle NetSuite: Full Asset Lifecycle
NetSuite's **Fixed Assets** module handles acquisition, depreciation (straight-line, declining balance, SUM-years-digits, units of production), revaluation, transfers, and disposals. **Automated depreciation runs** post journal entries automatically. **Asset groups** with batch updates. **Impairment tracking** for value write-downs. **Compliance** with GAAP, IFRS, and tax regulations.

### SAP S/4HANA: Enterprise Asset Finance
SAP's **Asset Accounting** is deeply integrated with FI and CO. **Parallel depreciation areas** for financial, tax, and group reporting. **Area-specific posting rules** for different accounting standards. **Integration with PM** (plant maintenance) for asset maintenance history. **Real-time valuation** with HANA.

### Odoo: Simple Depreciation
Odoo's asset module handles acquisition, depreciation (linear and declining), and disposal. **Automated depreciation entries** posted on period close. **Asset categories** define depreciation rules. **Depreciation boards** show schedule and actuals. Simple but effective for SMBs.

### QuickBooks Online: Basic Tracking
QBO tracks fixed assets with basic depreciation (straight-line only). **Depreciation journal entries** are manual. **Asset list** shows cost, accumulated depreciation, and book value. Limited: no declining balance, no revaluations, no disposals with gain/loss.

### Sage Intacct: Advanced Asset Mgmt
Sage Intacct supports **multiple depreciation methods**, **asset groups**, **component depreciation** (each part of an asset depreciated separately), **impairment tracking**, and **insurance value tracking**. **Asset disposal** calculates gain/loss automatically. **Integration with purchasing** for asset capitalization.

### Zoho Books: Basic Assets
Zoho Books offers **fixed asset register** with depreciation (straight-line only). **Monthly/annual depreciation** posting. **Disposal tracking** with gain/loss. Simple interface, limited methods. Good value for basic needs.

---

## Implementation Roadmap

### Phase 1: Asset & Category Model (2 weeks) — P1
1. Define `AssetCategory` entity with depreciation method and accounts
2. Define `Asset` entity with lifecycle states and depreciation tracking
3. Implement asset creation from acquisition data
4. Implement basic straight-line depreciation calculation

### Phase 2: Depreciation Engine (3 weeks) — P1
1. Implement depreciation methods (linear, declining balance, units of production)
2. Implement period-based depreciation run (single and batch)
3. Generate journal entries for depreciation automatically
4. Implement depreciation schedule view and report

### Phase 3: Revaluation & Disposal (2 weeks) — P2
1. Define `AssetRevaluation` and `AssetDisposal` entities
2. Implement revaluation with journal entry generation
3. Implement disposal with gain/loss calculation
4. Implement fixed asset register report

---

## Key Takeaway for Buyers

Fixed asset management ensures your balance sheet is accurate and compliant. A buyer should ask: *Can I track every asset, calculate depreciation automatically, and handle revaluations and disposals correctly?* RERP's API-first model means depreciation runs can be automated via API, and asset data is fully exportable. The gap with SAP/NetSuite is the depth of methods (component depreciation, parallel depreciation areas). But for organizations that need accurate depreciation with full API control, RERP delivers the foundation.

**The immediate priority: define Asset and AssetCategory entities with depreciation calculation. This is a legally required function for most businesses.**
