# intake/ — Multi-Channel Ingestion Gateway

**Path:** `documents/intake/`
**API:** `/api/v1/documents/intake/`

Receives documents from multiple channels and writes them into `core/`. The intake service is the **doorway** — once a document is in `core/`, the pipeline takes over.

## Intake Channels

All channels feed the same pipeline. Crucially, RERP supports **suite-level email routing** and **UI drag-and-drop**, so documents know their destination context before they're even processed.

| Channel | Example | User Experience |
|---|---|---|
| **Suite Email** | `invoices@company.rerp.local` | User emails PDF to a suite-specific alias; RERP routes to that suite's modules |
| **Camera/Mobile** | Snap invoice on phone | User takes photo → auto-uploads to RERP mobile app |
| **UI Drag-and-Drop** | Drag PDF onto sales page | User drags file onto a RERP frontend page; context is inherited from the page |
| **Webhook** | Third-party sends via API | External system POSTs document to RERP intake endpoint |
| **API** | Programmatic upload | Automated system submits documents via API |

## Suite-Level Email Routing

RERP assigns **one email alias per suite** (and optionally per module within a suite). When a document arrives via email, the address it was sent to determines the target suite and narrows the routing scope:

- One email alias per suite (`invoices@`, `contracts@`, etc.)
- Optional: module-level aliases within a suite (`invoices@ap.rerp.local`, `invoices@ar.rerp.local`)
- Aliases are configurable per tenant (each company sets their own email domain)
- BCC-based multi-routing: send to `general@` and BCC `invoices@` for double processing
- **No email address?** Documents without a suite-specific address (web upload, mobile, webhook) fall through to **AI classification + routing engine** which determines the target suite automatically.

## UI Drag-and-Drop Integration

When a user is working within a RERP module's frontend page, they can drag a document directly onto the page. The context of the page (which module, which record, which screen) is passed to the document pipeline:

**Examples across modules:**

| UI Page | User Action | Result |
|---|---|---|
| Sales → New Quote | Drag PDF catalog | Extract line items → pre-fill quote |
| CRM → New Lead | Drag business card | Extract contact info → pre-fill lead form |
| HR → New Employee | Drag offer letter + ID | Extract name, start date, salary → pre-fill profile |
| Inventory → Stock Receive | Drag receiving report | Extract SKU quantities → pre-fill receipt |
| Helpdesk → New Ticket | Drag support email text | Extract issue description → pre-fill ticket |
| Logistics → New Shipment | Drag shipping label | Extract weight, dimensions, destination → pre-fill |
| Project → New Task | Drag task brief | Extract description, deadline, assignee → pre-fill |

The drag-and-drop context also influences **schema selection** — if the user is on the accounting suite's invoice page, RERP uses the invoice schema by default rather than relying solely on AI classification. This reduces extraction errors.

## API Surface

```
POST   /intake/email                # Submit email document (for webhook-based forwarding)
POST   /intake/upload               # Upload document with metadata (writes to core/)
POST   /intake/url                  # Submit document by URL (RERP fetches it)
POST   /intake/batch                # Batch upload multiple documents
GET    /intake/email-aliases        # List configured email aliases
POST   /intake/email-aliases        # Create email alias
DELETE /intake/email-aliases/{id}    # Remove email alias
GET    /intake/mobile/status        # Mobile app sync status
```
