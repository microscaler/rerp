# Stock Movements & Transfers - Competitive Analysis

> **Component:** Stock Movements & Transfers
> **Microservice:** `inventory/stock-movements`
> **OpenAPI Spec:** [stock-movements/openapi.yaml](../../stock-movements/openapi.yaml)
> **Status:** Implemented

---

## Buyer Pitch

> *"When stock moves — into my warehouse, out to a customer, between locations — I need a complete audit trail with approval workflows and real-time stock synchronization."*

RERP Inventory's Stock Movements service tracks every inventory transaction: receipts from suppliers, dispatches to customers, internal transfers between locations, stock adjustments, and returns. Each movement goes through a pending → confirmed/cancelled workflow, providing the audit trail compliance demands require.

---

## Feature Comparison

| Feature | RERP Movements | SAP B1 | NetSuite | MS Dynamics 365 | QuickBooks Ent. | Odoo | Fishbowl | Acctivate | Cin7 |
|---------|---------------|--------|----------|-----------------|-----------------|------|----------|-----------|------|
| Movement types (receipt/dispatch/transfer) | ●●● | ●●○ | ●●● | ●●● | ●●○ | ●●● | ●●● | ●●● | ●●○ |
| Adjustment in/out | ●●● | ●●○ | ●●● | ●●● | ●○ | ●●○ | ●●● | ●●○ | ●●○ |
| Returns handling | ●●● | ●●○ | ●●● | ●●● | ●○○ | ●●○ | ●●● | ●●○ | ●●○ |
| Pending approval workflow | ●●● | ●○○ | ●●○ | ●●○ | ●○○ | ●○○ | ●○○ | ●○○ | ●○○ |
| Reference number linking (PO/SO) | ●●● | ●●● | ●●● | ●●● | ●●○ | ●●○ | ●●● | ●●● | ●●○ |
| Source/destination location tracking | ●●● | ●●○ | ●●● | ●●● | ●○○ | ●●● | ●●● | ●●○ | ●●○ |
| User attribution | ●●● | ●●○ | ●●● | ●●● | ●○○ | ●○ | ●●○ | ●○ | ●○ |
| Bulk movement operations | ●○○ | ●●○ | ●●● | ●●○ | ●○○ | ●●○ | ●●○ | ●●○ | ●●○ |
| Scheduled/recurring movements | ●○○ | ●●○ | ●●○ | ●●○ | ●○○ | ●○ | ●○ | ●○○ | ●○ |

**Legend:** ●●● = Full parity, ●●○ = Partial, ●○○ = Planned

---

## RERP vs. Competitors

### RERP Stock Movements Advantages
1. **Built-in approval workflow** — Every movement starts as "pending" requiring explicit confirmation. Competitors (SAP, NetSuite, QuickBooks) often auto-confirm, leaving no rollback capability.
2. **Full audit trail** — User attribution on every movement, complete history with timestamps. Most competitors lack per-movement user tracking.
3. **Movement cancellation** — Pending movements can be cancelled (not just deleted), preserving the audit trail. Competitors typically delete rather than cancel.
4. **Reference number linking** — Tie movements to PO numbers, SO numbers, or any external reference. Critical for reconciliation.
5. **Independent microservice** — Stock movements don't block stock availability queries. Competitors use shared database locks.

### Competitive Gaps
1. **No bulk movement operations** — NetSuite and SAP handle bulk stock adjustments for cycle counts. RERP must build batch endpoints.
2. **No scheduled movements** — NetSuite supports recurring transfers and adjustments. RERP must add this as a scheduling service.
3. **No mobile approval interface** — Warehouse supervisors need to confirm/cancel on mobile. RERP must build a companion mobile app.

---

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/inventory/stock-movements` | List all movements (paginated, searchable) |
| POST | `/api/v1/inventory/stock-movements` | Create a new movement |
| GET | `/api/v1/inventory/stock-movements/{id}` | Get movement details |
| PUT | `/api/v1/inventory/stock-movements/{id}` | Update movement (pending only) |
| DELETE | `/api/v1/inventory/stock-movements/{id}` | Delete movement (pending only) |
| POST | `/api/v1/inventory/stock-movements/{id}/confirm` | Confirm and finalize movement |
| POST | `/api/v1/inventory/stock-movements/{id}/cancel` | Cancel pending movement |

---

## Data Model

The Stock Movements service defines these core schemas:

- **StockMovement** — Movement record: ID, type, reference number, locations, status, user attribution, timestamps
- **Movement types:** receipt, dispatch, transfer, adjustment_in, adjustment_out, return
- **Movement statuses:** pending, confirmed, cancelled
- **CreateStockMovementRequest** / **UpdateStockMovementRequest** — Mutation inputs with line items
- **StockMovementItem** — Per-item movement: stock ID, quantity, unit cost
- **Confirm/Cancel endpoints** — Stateless operations that transition movement status

---

## Implementation Notes

- Movement workflow: create → pending → (confirm | cancel)
- Only pending movements can be updated or deleted
- Confirmed movements cannot be modified — only cancelled with a reversing movement
- All mutations include standardized error responses (400/401/403/409)
- `x-brrtrouter-impl: true` on all mutations for code generation
- UUID format for all resource IDs
- ISO 8601 date-time for timestamps
