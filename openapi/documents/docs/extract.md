# extract/ — Parse + Standardize

**Path:** `documents/extract/`
**API:** `/api/v1/documents/extract/`

Reads documents from `core/`, performs OCR and schema-based extraction, writes results back to `core/` metadata.

- **Parse** — OCR, table extraction, checkbox detection, handwriting recognition
- **Standardize** — Schema-based field extraction using JSON schema definitions
- **Auto-schema** — Upload sample docs, get proposed extraction schemas
- **Review highlights** — Bounding box coordinates for each extracted field

## API Surface

```
POST   /documents/{id}/parse           # Submit for OCR + table extraction
POST   /documents/{id}/standardize     # Extract using a schema
POST   /schemas                        # Create/edit extraction schema
POST   /schemas/generate               # Auto-generate from sample documents
GET    /schemas                        # List schemas
GET    /documents/{id}/extract/{sid}   # Get extraction results
GET    /documents/{id}/extract/{sid}/highlights  # Get bounding box coordinates
```
