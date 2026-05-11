# Logistics & Shipping — Carriers, Shipments, Rates

> Part of RERP Inventory Competitive Analysis
> See [main README](../README.md) for complete overview

---

## The Pitch

**"How do I find the cheapest shipping, print labels, and track deliveries?"**

Shipping is where inventory meets the customer. Businesses need to compare carrier rates, create shipments, print labels, and track packages — all integrated with their inventory system. RERP's logistics service handles carrier management, rate shopping, and shipment tracking.

---

## What Buyers Compare

### Carrier Management
- **Multi-carrier support** — UPS, FedEx, DHL, USPS, regional carriers
- **Carrier profiles** — Account numbers, API keys, service levels per carrier
- **Rate comparison** — Real-time rate shopping across carriers for each package
- **Service level selection** — Ground, 2-day, overnight, economy options

### Shipment Management
- **Label generation** — Print shipping labels with tracking numbers
- **Package dimensions** — Length, width, height, weight for accurate rates
- **Shipment tracking** — Real-time status updates from carrier APIs
- **Proof of delivery** — Signature capture, delivery photos, confirmation

### Rate Management
- **Rate cards** — Pre-negotiated carrier rates by zone, weight, and service
- **Discount application** — Carrier discounts and surcharges (residential, fuel)
- **Rate comparison engine** — Automatic carrier selection based on cost or speed
- **Freight cost allocation** — Assign shipping costs to orders or customers

---

## Competitor Comparison

| Feature | RERP | Odoo | NetSuite | SAP | MS Dynamics | Fishbowl |
|---------|------|------|----------|-----|-------------|----------|
| Multi-carrier | ●○○ | ●●○ | ●●● | ●●● | ●●○ | ●○ |
| Rate shopping | ●○○ | ●○ | ●●○ | ●●● | ●○ | ●○ |
| Label generation | ●○○ | ●○ | ●●○ | ●●● | ●○ | ●○ |
| Shipment tracking | ●○○ | ●○ | ●●○ | ●●● | ●○ | ●○ |
| Proof of delivery | ●○○ | ●○ | ●●○ | ●●● | ●○ | ●○ |
| Rate comparison | ●○○ | ●○ | ●●○ | ●●● | ●○ | ●○ |
| Freight cost allocation | ●○○ | ●○ | ●●○ | ●●● | ●○ | ●○ |

---

## RERP's Approach

### Core Entities
- **Shipment** — Outbound shipment with carrier, tracking, and status
- **Carrier** — Carrier profile with account details and enabled services
- **ShippingRate** — Pre-negotiated or negotiated carrier rates

### API Endpoints
```
GET    /shipments                     — List shipments (paginated)
POST   /shipments                     — Create shipment
GET    /shipments/{id}                — Get shipment by ID
PUT    /shipments/{id}                — Update shipment
DELETE /shipments/{id}                — Delete shipment

GET    /carriers                      — List carriers (paginated)
POST   /carriers                      — Create carrier
GET    /carriers/{id}                 — Get carrier by ID
PUT    /carriers/{id}                 — Update carrier
DELETE /carriers/{id}                — Delete carrier

GET    /shipping-rates                — List rates (paginated)
POST   /shipping-rates                — Create rate
GET    /shipping-rates/{id}           — Get rate by ID
PUT    /shipping-rates/{id}           — Update rate
DELETE /shipping-rates/{id}           — Delete rate
```

### Competitive Advantage
- **API-first carrier integration** — Clean REST API for any carrier integration
- **Rust performance** — Fast rate calculations and real-time tracking updates
- **Self-hosted** — No transaction fees per shipment or label

### Gap to Close
- Carrier API integrations (UPS, FedEx, DHL, USPS)
- Label printing (PDF/ZPL generation)
- Rate shopping algorithm with carrier discount support
- Tracking webhook handlers for real-time status updates
- Freight cost allocation engine

---

*Component directory: [Inventory Core](../core/README.md) | [Warehouse Operations](../warehouse-operations/README.md) | [Dropshipping](../dropshipping/README.md)*
