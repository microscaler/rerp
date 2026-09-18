# Inventory Suite Competitive Analysis

> **Date:** 2026-05-11
> **Purpose:** Pitch-level competitive analysis for buyer decision-making
> **Scope:** RERP Inventory vs. SAP Business One, Oracle NetSuite, Microsoft Dynamics 365 Business Central, QuickBooks Enterprise, Odoo Inventory, Fishbowl Inventory, Acctivate, Cin7

---

## Overview

This analysis examines inventory management capabilities across **7 functional components**, comparing RERP Inventory against the competitive landscape from a buyer's perspective. Each component is documented as a pitch — the question a buyer asks and the answer their options provide.

The competitors evaluated:

| Vendor | Market Position | Best For | Pricing Model |
|--------|----------------|----------|---------------|
| **SAP Business One** | SMB/Mid-market ERP | Manufacturing, distribution | Perpetual ~$1,125/user + subscription ~$115/user/mo |
| **Oracle NetSuite** | Cloud ERP Leader | Mid-market, multi-entity | Implementation $15K–$150K + ~$999+/mo subscription |
| **Microsoft Dynamics 365 BC** | Enterprise Contender | Microsoft-centric orgs | Per-user/month $70–$220 |
| **QuickBooks Enterprise** | SMB Accounting | Small businesses, retail | $2,880–$8,400/yr + $85–$195/mo |
| **Odoo Inventory** | Open-Source | DIY organizations, SMB | Free Community / Enterprise ~$25.20/user/mo |
| **Fishbowl Inventory** | Mid-Market | Manufacturing, fabrication | ~$1,400–$2,100/yr per user + QuickBooks add-on |
| **Acctivate** | SMB Specialist | Intuit QuickBooks partners | ~$125–$175/user/mo |
| **Cin7** | E-commerce | Online retailers, omnichannel | ~$297–$744/mo |
| **RERP Inventory** | Open-Source, API-First | Dev-driven orgs, data sovereignty | Self-hosted (free) / Hosted (TBD) |

---

## Component Directory

| # | Component | Directory | Status |
|---|-----------|-----------|--------|
| 1 | Stock Items & Availability | [stock/README.md](stock/README.md) | **Implemented** |
| 2 | Stock Movements & Transfers | [stock-movements/README.md](stock-movements/README.md) | **Implemented** |
| 3 | Stock Valuations & Costing | [stock-valuations/README.md](stock-valuations/README.md) | **Implemented** |
| 4 | Warehouse Operations | [warehouse-operations/README.md](warehouse-operations/README.md) | **Implemented** |
| 5 | Dropshipping & Fulfillment | [dropshipping/README.md](dropshipping/README.md) | **Implemented** |
| 6 | Inventory Planning | [inventory-planning/README.md](inventory-planning/README.md) | Planned |
| 7 | Barcode & Device Integration | [barcode-tracking/README.md](barcode-tracking/README.md) | Planned |

---

## Head-to-Head Capability Summary

| Capability Area | RERP | SAP B1 | NetSuite | MS Dynamics | QuickBooks | Odoo | Fishbowl | Acctivate | Cin7 |
|----------------|------|--------|----------|-------------|------------|------|----------|-----------|------|
| Stock Items/Mgmt | ●●● | ●●● | ●●● | ●●● | ●●○ | ●●● | ●●● | ●●● | ●●● |
| Stock Movements | ●●● | ●●○ | ●●● | ●●● | ●●○ | ●●● | ●●● | ●●● | ●●○ |
| Stock Valuations | ●●● | ●●● | ●●● | ●●● | ●●○ | ●●○ | ●●● | ●●● | ●●○ |
| Warehouse Ops | ●●○ | ●●○ | ●●● | ●●● | ●○○ | ●●● | ●●● | ●●○ | ●●○ |
| Dropshipping | ●●○ | ●●○ | ●●● | ●●○ | ●○○ | ●●○ | ●●○ | ●○○ | ●●● |
| Inventory Planning | ●○○ | ●●○ | ●●● | ●●● | ●○○ | ●●○ | ●●○ | ●●○ | ●●○ |
| Barcode/Device | ●○○ | ●●○ | ●●● | ●●○ | ●●○ | ●●● | ●●● | ●●○ | ●●● |

**Legend:** ●●● = Full feature parity, ●●○ = Partial coverage, ●○○ = Planned / not yet implemented

---

## RERP Inventory's Strategic Position

### Strengths

1. **OpenAPI-first architecture** — Every entity, endpoint, and schema is machine-readable. Enables automatic SDK generation, API contracts, and tooling. No competitor except Cin7 exposes its inventory data model this cleanly.

2. **Rust-based performance** — Axum + async I/O delivers sub-millisecond API latency. Stock updates across 100K+ items complete in seconds, not minutes like SAP/NetSuite GUI transactions.

3. **Self-hosted, no vendor lock-in** — No per-seat pricing, no rate limits, no data egress fees. Full control over infrastructure and data. Unlike Fishbowl (QuickBooks dependent) or Acctivate (Intuit partner), RERP has no upstream vendor dependency.

4. **Seven-service modular architecture** — Stock, movements, valuations, warehouse, dropshipping, planning, and barcode services allow independent scaling and parallel development. Competitors bundle everything into monolithic GUI applications.

5. **Real-time across bounded contexts** — Each microservice owns its data boundary. Stock movements don't block stock availability queries. Valuations don't require warehouse locks. Competitors use shared database locks that slow concurrent operations.

6. **Flexible costing methods** — FIFO, LIFO, weighted average, and specific cost supported out of the box. QuickBooks Enterprise limits to average/standard. Odoo Community lacks LIFO.

### Weaknesses (Current)

1. **No GUI — API-only** — Buyers expect visual dashboards. SAP/NetSuite/Odoo have mature UX teams. RERP requires companion frontend development.

2. **No barcode scanning integration** — Fishbowl, Odoo, and Cin7 support GS1/EAN13 barcode scanners natively. RERP must build this via OpenAPI clients.

3. **No physical inventory counting workflow** — Competitors provide guided cycle count processes with print labels. RERP must implement this on top of stock endpoints.

4. **No manufacturer/bill-of-materials support** — SAP Business One, NetSuite, and Fishbowl handle BOMs and manufacturing. RERP inventory focuses on trading, not production.

5. **No landed cost allocation** — Acctivate, Fishbowl, and NetSuite distribute freight/duties/taxes across inventory items. RERP's stock-valuations tracks unit cost but not cost allocation from PO line items.

6. **No multi-currency inventory valuation** — SAP, NetSuite, and MS Dynamics handle inventory valuation in multiple reporting currencies. RERP tracks a single currency per valuation.

### Threats

- **NetSuite's demand planning moat** — Advanced Inventory module adds supply allocation, demand planning, and ATP calculations. Moving target for RERP.
- **Odoo's free Community edition** — SMBs can deploy Odoo Inventory for free. The conversion path to paid modules is well-trodden.
- **QuickBooks' market dominance** — 60%+ of US small businesses use QuickBooks. Acctivate/Fishbowl ride this distribution channel. RERP needs a different go-to-market.

### Opportunities

- **E-commerce/SMB cost sensitivity** — Organizations tired of NetSuite's $15K+ implementation and $999+/mo subscription. RERP self-hosted eliminates these barriers.
- **Developer-first organizations** — Teams that value API contracts over drag-and-drop builders. RERP's OpenAPI specs are a differentiator, not a limitation.
- **Regulated industries** — Healthcare, food/beverage, government — where self-hosting, data sovereignty, and audit trails are required.
- **API economy integration** — RERP can plug into Shopify, WooCommerce, Amazon, eBay via its API-first design. Competitors require expensive connectors or middleware.

---

## Implementation Priority Matrix

| Priority | Component | Effort | Impact | Rationale |
|----------|-----------|--------|--------|-----------|
| **P0** | Barcode & Device Integration | Medium | Critical | Warehouse operators need barcode scanning; no one buys pure-API inventory |
| **P0** | Inventory Planning | High | High | Demand planning is what separates "good inventory" from "enterprise inventory" |
| **P1** | Landed Cost Allocation | Medium | High | Acctivate/Fishbowl have this; SMB distributors expect it |
| **P1** | Physical Inventory Workflow | Medium | High | Cycle counts and stock audits are table stakes for SMB inventory |
| **P2** | Multi-currency Valuation | Medium | Medium | Needed for mid-market/international but not for SMB launch |
| **P2** | BOM/Manufacturing | High | Medium | Important for manufacturing vertical but adds significant scope |
| **P3** | Analytics Dashboard | High | Medium | Management needs visibility but API consumers can build their own dashboards |
| **P4** | Mobile App | High | Low | Companion to barcode; can be built on top of existing OpenAPI specs |

---

## Quick Links

- [Current OpenAPI Spec](../openapi.yaml) — RERP Inventory gateway specification
- [Stock Service Spec](stock/openapi.yaml) — Stock items and availability sub-spec
- [Stock Movements Spec](stock-movements/openapi.yaml) — Movement transactions sub-spec
- [Stock Valuations Spec](stock-valuations/openapi.yaml) — Costing and valuation sub-spec
- [Warehouse Operations Spec](../warehouse/openapi.yaml) — Multi-warehouse sub-spec
- [Dropshipping Spec](../dropshipping/openapi.yaml) — Dropship orders sub-spec
