# Stock Valuations & Costing - Competitive Analysis

> **Component:** Stock Valuations & Costing
> **Microservice:** `inventory/stock-valuations`
> **OpenAPI Spec:** [stock-valuations/openapi.yaml](../../stock-valuations/openapi.yaml)
> **Status:** Implemented

---

## Buyer Pitch

> *"I need to know the true cost of my inventory — what I paid, what it's worth now, and how valuation method affects my margin reporting."*

RERP Inventory's Stock Valuations service handles inventory costing with support for FIFO, LIFO, weighted average, and specific cost methods. It provides real-time valuation calculations, manual cost override capabilities, and a complete audit trail of cost changes. Critical for accurate financial reporting and margin analysis.

---

## Feature Comparison

| Feature | RERP Valuations | SAP B1 | NetSuite | MS Dynamics 365 | QuickBooks Ent. | Odoo | Fishbowl | Acctivate | Cin7 |
|---------|----------------|--------|----------|-----------------|-----------------|------|----------|-----------|------|
| FIFO costing | ●●● | ●●● | ●●● | ●●● | ●●○ | ●●○ | ●●● | ●●● | ●●○ |
| LIFO costing | ●●● | ●●○ | ●●● | ●●○ | ●○○ | ●○○ | ●●● | ●●● | ●○○ |
| Weighted average | ●●● | ●●● | ●●● | ●●● | ●●○ | ●●○ | ●●● | ●●● | ●●○ |
| Specific cost | ●●● | ●●○ | ●●● | ●●○ | ●○○ | ●○○ | ●●○ | ●○ | ●○○ |
| Manual cost override | ●●● | ●○○ | ●●○ | ●●○ | ●○○ | ●○○ | ●○ | ●●○ | ●○○ |
| Cost change history | ●●● | ●●○ | ●●○ | ●●○ | ●○○ | ●○○ | ●○ | ●●○ | ●○○ |
| Multi-currency valuation | ●○○ | ●●● | ●●● | ●●● | ●○ | ●○ | ●○ | ●○ | ●○ |
| Standard cost management | ●○○ | ●●○ | ●●● | ●●○ | ●●○ | ●○ | ●○ | ●○ | ●○○ |
| Landed cost allocation | ●○○ | ●●○ | ●●● | ●●○ | ●○○ | ●○○ | ●●● | ●●● | ●○ |
| Variance analysis | ●○○ | ●●○ | ●●● | ●●○ | ●○○ | ●○○ | ●○ | ●●○ | ●○○ |

**Legend:** ●●● = Full parity, ●●○ = Partial, ●○○ = Planned

---

## RERP vs. Competitors

### RERP Stock Valuations Advantages
1. **Four costing methods** — FIFO, LIFO, weighted average, and specific cost. QuickBooks only supports average/standard. Odoo Community lacks LIFO. This breadth of methods gives accounting teams flexibility.
2. **Manual cost override** — When market prices change, RERP allows direct cost updates with reason tracking. Most competitors only update on receipt.
3. **Complete cost change history** — Every cost change is tracked with timestamps. Competitors often store only current cost.
4. **Independent microservice** — Valuation calculations don't block stock availability queries. Competitors share database transactions that create contention.
5. **Cost endpoint** — Dedicated `/cost` endpoint for getting current valuation using the configured method. Clean separation from general valuation records.

### Competitive Gaps
1. **No multi-currency valuation** — SAP, NetSuite, and MS Dynamics handle inventory valuation in multiple reporting currencies. RERP tracks a single currency per valuation. Critical for international businesses.
2. **No standard cost management** — NetSuite supports standard vs. actual cost variance tracking. RERP must build this for manufacturing customers.
3. **No landed cost allocation** — Acctivate and Fishbowl distribute freight, duties, and taxes across inventory items from PO line items. RERP's stock-valuations tracks unit cost but not cost allocation.
4. **No variance analysis** — NetSuite and SAP provide standard cost vs. actual cost variance reporting. Important for manufacturing margin analysis.

---

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/inventory/stock-valuations` | List all valuations (paginated, searchable) |
| POST | `/api/v1/inventory/stock-valuations` | Create a new valuation |
| GET | `/api/v1/inventory/stock-valuations/{id}` | Get valuation details |
| PUT | `/api/v1/inventory/stock-valuations/{id}` | Update valuation |
| DELETE | `/api/v1/inventory/stock-valuations/{id}` | Delete valuation |
| GET | `/api/v1/inventory/stock-valuations/{stock_id}/cost` | Get current cost using configured method |
| GET | `/api/v1/inventory/valuations/methods` | Get available valuation methods |
| PUT | `/api/v1/inventory/valuations/methods/{method_id}` | Update valuation method |

---

## Data Model

The Stock Valuations service defines these core schemas:

- **StockValuation** — Valuation record: stock ID, method ID, last cost, total value, quantity on hand, currency
- **StockCurrentCost** — Current cost derived from valuation method: unit cost, total value, method used
- **ValuationMethod** — Configured costing method: FIFO, LIFO, weighted average, specific cost
- **CreateStockValuationRequest** / **UpdateStockValuationRequest** — Mutation inputs
- **UpdateStockCostRequest** — Manual cost override with reason
- **UpdateValuationMethodRequest** — Method change with effective date

---

## Implementation Notes

- Valuation methods: fifo, lifo, weighted_average, specific_cost
- Cost endpoint calculates current cost based on active valuation method
- Manual cost overrides tracked separately from automatic cost updates
- All mutations include standardized error responses (400/401/403/409)
- `x-brrtrouter-impl: true` on all mutations for code generation
- UUID format for all resource IDs
- ISO 8601 date-time for timestamps
- Decimal precision for cost fields (number type with format: double)
