# Dropshipping — Vendor Orders & Dropship Fulfillment

> Part of RERP Inventory Competitive Analysis
> See [main README](../README.md) for complete overview

---

## The Pitch

**"How do I fulfill orders from vendors without holding stock?"**

Dropshipping lets you sell products you don't physically stock. When a customer buys, you automatically place an order with the vendor who ships directly to the customer. RERP automates this entire flow: vendor order creation, tracking updates, and fulfillment status sync.

---

## What Buyers Compare

### Dropship Order Management
- **Order creation** — Convert customer orders into vendor dropship orders
- **Vendor assignment** — Route orders to the right vendor based on product or location
- **Order tracking** — Real-time tracking from vendor through to customer
- **Partial fulfillment** — Handle orders where vendors ship in multiple shipments
- **Return processing** — Vendor return authorization (RMA) workflows

### Vendor Order Management
- **Purchase orders** — Create and send POs to vendors with product details
- **Vendor portals** — Vendor confirmation and shipment updates
- **PO status tracking** — Draft, sent, confirmed, shipped, received, completed
- **Vendor performance** — On-time delivery rates, quality metrics

### Automation
- **Auto-order creation** — Automatic PO generation when customer orders dropship items
- **Tracking sync** — Automatic tracking number capture and customer notification
- **Profit margin tracking** — Sell price vs. vendor cost per order

---

## Competitor Comparison

| Feature | RERP | Odoo | NetSuite | SAP | Fishbowl | Acctivate |
|---------|------|------|----------|-----|----------|-----------|
| Dropship orders | ●○○ | ●●○ | ●●● | ●●○ | ●○ | ●○ |
| Vendor POs | ●○○ | ●●○ | ●●● | ●●○ | ●○ | ●○ |
| Order tracking sync | ●○○ | ●○ | ●●○ | ●●○ | ●○ | ●○ |
| Auto-order creation | ●○○ | ●○ | ●●○ | ●●○ | ●○ | ●○ |
| Partial fulfillment | ●○○ | ●○ | ●●○ | ●●○ | ●○ | ●○ |
| Return/RMA | ●○○ | ●○ | ●●○ | ●●○ | ●○ | ●○ |
| Vendor performance | ●○○ | ●○ | ●●○ | ●●○ | ●○ | ●○ |

---

## RERP's Approach

### Core Entities
- **DropshipOrder** — Customer-facing order with vendor fulfillment details
- **VendorOrder** — Purchase order sent to vendor for fulfillment

### API Endpoints
```
GET    /dropship-orders               — List dropship orders (paginated)
POST   /dropship-orders               — Create dropship order
GET    /dropship-orders/{id}          — Get order by ID
PUT    /dropship-orders/{id}          — Update order
DELETE /dropship-orders/{id}          — Delete order

GET    /vendor-orders                 — List vendor orders (paginated)
POST   /vendor-orders                 — Create vendor order
GET    /vendor-orders/{id}            — Get vendor order by ID
PUT    /vendor-orders/{id}            — Update vendor order
DELETE /vendor-orders/{id}            — Delete vendor order
```

### Competitive Advantage
- **API-first vendor integration** — Clean REST API for vendor portal or EDI integration
- **Rust performance** — Real-time order status updates across many vendors
- **Self-hosted** — No transaction fees on dropship orders

### Gap to Close
- Vendor portal or EDI integration layer
- Automatic tracking number capture from carrier APIs
- Customer notification workflows (email/SMS)
- Return/RMA vendor workflow
- Profit margin reporting per dropship order

---

*Continue to [Logistics & Shipping](../logistics-shipping/README.md)*
