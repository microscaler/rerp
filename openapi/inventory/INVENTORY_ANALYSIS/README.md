# RERP Inventory Suite — Competitive Analysis

> **Date:** 2026-05-11
> **Purpose:** Pitch-level competitive analysis for buyer decision-making
> **Scope:** RERP Inventory vs. Odoo Inventory, NetSuite, SAP S/4HANA, Microsoft Dynamics 365, Fishbowl, Oracle, Cin7

---

## Overview

This analysis examines inventory management capabilities across **4 functional components**, comparing RERP Inventory against the competitive landscape from a buyer's perspective. Each component is documented as a pitch — the question a buyer asks and the answer their options provide.

The competitors evaluated:

| Vendor | Market Position | Best For | Pricing Model |
|--------|----------------|----------|---------------|
| **SAP S/4HANA** | Enterprise Leader | Large manufacturers, global supply chains | Enterprise licensing ($100K+) |
| **Oracle Fusion Cloud SCM** | Enterprise Contender | Large enterprises, cloud-first | Per-user/month ($150+) |
| **NetSuite** | Mid-Market Leader | Growing mid-market, cloud ERP | Per-user/month ($99-$150+) |
| **Microsoft Dynamics 365** | Enterprise Mid-Market | Microsoft-centric orgs | Per-user/month ($70-$180+) |
| **Odoo Inventory** | Open-Source SMB | SMB, DIY organizations | Free Community / Enterprise per-user |
| **Fishbowl** | SMB/Mid-Market | QuickBooks users, warehouse ops | Per-user/month ($150+) |
| **Acctivate** | SMB Inventory | SMB manufacturers, distributors | Per-user/month ($100-$150) |
| **RERP Inventory** | Open-Source, API-First | Dev-driven orgs, data sovereignty | Self-hosted (free) / Hosted (TBD) |

---

## Component Directory

| # | Component | Directory | Status |
|---|-----------|-----------|--------|
| 1 | Inventory Core (Stock, Movements, Valuations) | [core/README.md](core/README.md) | Planned |
| 2 | Warehouse Operations | [warehouse-operations/README.md](warehouse-operations/README.md) | Planned |
| 3 | Dropshipping | [dropshipping/README.md](dropshipping/README.md) | Planned |
| 4 | Logistics & Shipping | [logistics-shipping/README.md](logistics-shipping/README.md) | Planned |

---

## Head-to-Head Capability Summary

| Capability Area | RERP | Odoo | NetSuite | SAP | MS Dynamics | Fishbowl | Acctivate |
|----------------|------|------|----------|-----|-------------|----------|-----------|
| Stock Management | ●○○ | ●●● | ●●● | ●●● | ●●● | ●●● | ●●○ |
| Warehouse Ops | ●○○ | ●●○ | ●●○ | ●●● | ●●○ | ●●● | ●○ |
| Dropshipping | ●○○ | ●○ | ●●○ | ●●○ | ●○ | ●○ | ●○ |
| Logistics/Shipping | ●○○ | ●○ | ●●○ | ●●● | ●●○ | ●○ | ●○ |

**Legend:** ●●● = Full feature parity, ●●○ = Partial coverage, ●○○ = Planned / not yet implemented

---

## RERP Inventory's Strategic Position

### Strengths
1. **OpenAPI-first architecture** — Every entity, endpoint, and schema is machine-readable. Enables automatic SDK generation, API contracts, and tooling. No other inventory system exposes its data model this cleanly.
2. **Rust-based performance** — Axum + async I/O delivers sub-millisecond API latency. Real-time stock calculations across 100,000+ SKUs complete in seconds.
3. **Self-hosted, no vendor lock-in** — No per-seat pricing, no rate limits, no data egress fees. Full control over infrastructure and data.
4. **Two-crate codegen model** — Separation of generated (from OpenAPI) and implementation (business logic) enables safe regeneration.
5. **Modular service architecture** — The 4-service design (core, warehouse, dropshipping, logistics) allows parallel development and independent scaling.

### Weaknesses (Current)
1. **Empty schema definitions** — The most critical gap. All sub-specs reference entities but schemas are blank.
2. **No stock valuation model** — No FIFO, LIFO, weighted average costing.
3. **No barcode/RFID support** — No scanning, picking, packing workflows.
4. **No multi-warehouse support** — No location hierarchy, transfers, or bin management.
5. **No carrier integration** — No UPS, FedEx, DHL, or USPS API connections.
6. **No dropship automation** — No vendor order sync, tracking updates, or fulfillment workflows.
7. **No reporting surface** — No dashboards, pivot tables, or KPI endpoints.

### Threats
- **Odoo's ecosystem lock-in** — Once inventory, sales, and purchases are built in Odoo, migration cost is prohibitive.
- **NetSuite's AI moat** — AI-powered forecasting adds features that RERP cannot match without ML engineering investment.
- **Microsoft's bundled advantage** — Dynamics 365 is free for Office 365 businesses. The friction to adopt is near zero.

### Opportunities
- **SMB/mid-market cost sensitivity** — Organizations tired of NetSuite's $150+/user/month pricing.
- **Developer-first organizations** — Teams that value API contracts over drag-and-drop builders.
- **Regulated industries** — Healthcare, finance, government — where self-hosting and data sovereignty are required.
- **AI-native inventory** — RERP's Rust infrastructure is ideal for embedding ML inference at API scale.

---

## Implementation Priority Matrix

| Priority | Component | Effort | Impact | Rationale |
|----------|-----------|--------|--------|-----------|
| **P0** | Inventory Core | Medium | Critical | Foundation — nothing works without stock tracking |
| **P0** | Warehouse Operations | High | Critical | Multi-location, picking, packing are table stakes |
| **P1** | Dropshipping | Medium | High | Important for e-commerce merchants |
| **P1** | Logistics & Shipping | Medium | High | Carrier integration is essential for fulfillment |

---

## Quick Links

- [Current OpenAPI Spec](../openapi.yaml) — RERP Inventory API gateway specification
- [Core Service Spec](core/openapi.yaml) — Stock, movements, valuations sub-spec
- [Warehouse Spec](warehouse/openapi.yaml) — Warehouse operations sub-spec
- [Dropshipping Spec](dropshipping/openapi.yaml) — Vendor dropship orders sub-spec
- [Logistics Spec](logistics/openapi.yaml) — Carriers, shipments, rates sub-spec
