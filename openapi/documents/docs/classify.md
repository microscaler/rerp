# classify/ — Document Classification

**Path:** `documents/classify/`
**API:** `/api/v1/documents/classify/`

AI classification into document types. Pre-built classifiers for common document types + custom classifier training. Writes classification results to `core/` metadata.

## Pre-built Classifiers (Mapped to RERP Modules)

| Document Type | Target Module |
|---|---|
| Invoice | accounts-payable / accounts-receivable |
| Purchase Order | purchase-orders |
| Receipt | accounting (expense) |
| Quote / Estimate | sales |
| Contract | legal / contracts |
| Lease | real-estate |
| Business Card | crm (contact) |
| Resume | hr-recruitment |
| Payslip | hr-payroll |
| ID Document | hr (employee profile) |
| Timesheet | hr-timesheets |
| Medical Certificate | hr-leave |
| Shipping Document | logistics |
| Receiving Report | inventory |
| Quality Report | quality |
| Bank Statement | accounting (reconciliation) |
| Tax Document | accounting (tax) |
| Product Catalog | product-catalog |
| BOM Document | manufacturing |
| Project Proposal | project-management |
| Support Email | helpdesk |
| Service Report | field-service |
| Campaign Brief | marketing |
| POS Receipt | pos |
| Return Receipt | pos (returns) |

## API Surface

```
POST   /documents/{id}/classify      # Classify a document
POST   /classifiers                  # Create custom classifier
GET    /classifiers                  # List classifiers
GET    /documents/{id}/classification # Get classification result
```
