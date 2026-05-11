# Inventory Core — Stock, Movements, Valuations

> Part of RERP Inventory Competitive Analysis
> See [main README](../README.md) for complete overview

---

## The Pitch

**"How do I track what I have, where it is, and what it's worth?"**

Every inventory system starts with three questions: What's in stock? What moved? How much is it worth? RERP's core service answers all three with real-time stock levels, complete audit trails, and accurate cost valuations.

---

## What Buyers Compare

### Stock Management
- **Real-time stock levels** — Track quantities per SKU, per location, per status (available, reserved, damaged)
- **SKU management** — Product codes, descriptions, categories, units of measure
- **Low-stock alerts** — Automatic notifications when quantities fall below thresholds
- **Stock adjustments** — Manual corrections with reason codes and audit trails

### Stock Movements
- **Complete audit trail** — Every transaction recorded: receipts, issues, transfers, adjustments
- **Movement types** — Purchase receipt, sales issue, internal transfer, adjustment, return
- **Batch/lot tracking** — Track by lot number, serial number, or expiry date
- **Timestamped transactions** — Date, time, user, and source system for every movement

### Stock Valuations
- **Cost methods** — FIFO, LIFO, weighted average, standard cost
- **Multi-currency** — Track costs in multiple currencies with FX conversion
- **Inventory valuation reports** — Current value by SKU, category, warehouse, or location
- **Revaluation entries** — Periodic cost adjustments with GL integration

---

## Competitor Comparison

| Feature | RERP | Odoo | NetSuite | SAP | Fishbowl | Acctivate |
|---------|------|------|----------|-----|----------|-----------|
| Real-time stock | ●○○ | ●●● | ●●● | ●●● | ●●● | ●●○ |
| Stock adjustments | ●○○ | ●●● | ●●● | ●●● | ●●● | ●●○ |
| Low-stock alerts | ●○○ | ●●○ | ●●○ | ●●● | ●●○ | ●○ |
| Movement audit trail | ●○○ | ●●○ | ●●● | ●●● | ●●● | ●●○ |
| FIFO cost method | ●○○ | ●●○ | ●●● | ●●● | ●●○ | ●○ |
| Weighted avg cost | ●○○ | ●○ | ●●○ | ●●● | ●○ | ●○ |
| Multi-currency costing | ●○○ | ●●○ | ●●● | ●●● | ●○ | ●○ |
| Lot/serial tracking | ●○○ | ●●○ | ●●● | ●●● | ●●● | ●●○ |

---

## RERP's Approach

### Core Entities
- **Stock** — Current quantity, reserved quantity, available quantity per SKU
- **StockMovement** — Transaction record: type, quantity, source, destination, timestamp
- **StockValuation** — Cost per unit, total value, cost method applied

### API Endpoints
```
GET    /stocks                      — List all stocks (paginated)
POST   /stocks                      — Create stock item
GET    /stocks/{id}                 — Get stock by ID
PUT    /stocks/{id}                 — Update stock item
DELETE /stocks/{id}                 — Delete stock item

GET    /stock-movements             — List movements (paginated)
POST   /stock-movements             — Create movement
GET    /stock-movements/{id}        — Get movement by ID
PUT    /stock-movements/{id}        — Update movement
DELETE /stock-movements/{id}        — Delete movement

GET    /stock-valuations            — List valuations (paginated)
POST   /stock-valuations            — Create valuation
GET    /stock-valuations/{id}       — Get valuation by ID
PUT    /stock-valuations/{id}       — Update valuation
DELETE /stock-valuations/{id}       — Delete valuation
```

### Competitive Advantage
- **Rust performance** — Stock calculations across 100K+ SKUs in milliseconds
- **API-first design** — Every entity exposed via OpenAPI, enabling custom integrations
- **Self-hosted** — No per-user fees, no data egress, complete data sovereignty

### Gap to Close
- Stock valuation engine (FIFO/LIFO/weighted average)
- Batch/lot/serial number tracking
- Low-stock alerting system
- GL integration for valuation entries

---

*Continue to [Warehouse Operations](../warehouse-operations/README.md)*
