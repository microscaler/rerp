# confirmation/ — User Verification

**Path:** `documents/confirmation/`
**API:** `/api/v1/documents/confirmation/`

User review and approval workflow for extracted data before it's committed to RERP modules. References documents from `core/` for the review UI.

## API Surface

```
GET    /confirmations/pending          # List pending confirmations
GET    /confirmations/{id}             # Get confirmation details
POST   /confirmations/{id}/approve     # Approve (triggers target module API)
POST   /confirmations/{id}/reject      # Reject (returns to manual review)
POST   /confirmations/{id}/correct     # Correct (updates schema/ML model)
POST   /confirmations/{id}/batch-approve  # Approve multiple at once
GET    /confirmations/rules            # List auto-approve rules
POST   /confirmations/rules            # Create auto-approve rule
```

## Confirmation UI Concept

The confirmation UI presents:
- **Document Preview** — original document from core/
- **Extracted Data** — classification, confidence score, target module, extracted fields
- **User Actions** — Approve & Create Record, Reject, Edit/Correct
